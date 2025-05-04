use crate::domain::metadata::aggregate::Aggregate;
use crate::domain::metadata::catalog::Catalog;
use crate::domain::metadata::database_metadata::DatabaseMetadata;
use crate::domain::metadata::rls_policy::RlsPolicy;
use crate::domain::metadata::sequence::Sequence;
use crate::domain::metadata::{
    function::Function, materalized_view::MateralizedView, procedure::Procedure, table::Table,
    trigger, view::View,
};
use crate::domain::repository::database_repository::DatabaseRepository;
use dashmap::mapref::one::Ref;
use rbdc_oracle::driver::OracleDriver;

//use rbdc_oracle::*;
use rbs::value::map::ValueMap;
use rbs::Value;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

use crate::domain::metadata::database::Schema;
use crate::DatabaseConnection;
use dashmap::DashMap;

use crate::domain::metadata::utype::TypeField;

#[derive(Clone)]
pub struct OracleRepository {
    pub rb_map: DashMap<String, Arc<rbatis::RBatis>>,
    base_url: String,
    pub databases: DashMap<String, DatabaseMetadata>,
    connection_info: DatabaseConnection,
}

impl OracleRepository {
    pub fn new(connection_info: DatabaseConnection) -> Self {
        let rb_map = DashMap::new();
        let databases = DashMap::new();
        let base_url = String::from(format!(
            "{}://{}:{}@{}:{}/oracle",
            connection_info.driver_type,
            connection_info.username,
            connection_info.password,
            connection_info.server,
            connection_info.port
        ));
        return OracleRepository {
            rb_map,
            base_url,
            databases,
            connection_info,
        };
    }

    ///Add the database to the pool if not exists
    ///It's create a new rbatis, initialize it add add to the pool
    async fn connect(&self, db_name: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !self.rb_map.contains_key(db_name) {
            log::info!("new pool adding... database: {:?}", db_name);
            let rb = Arc::new(rbatis::RBatis::new());
            match rb.init(OracleDriver {}, url) {
                Ok(_) => log::info!("Connection to {} successful", db_name),
                Err(e) => {
                    log::error!("Failed to initialize rbatis for {}: {:?}", db_name, e);
                    return Err(Box::new(e)); // Return the error early
                }
            }
            self.rb_map.insert(db_name.to_string(), rb);
            let db_node = Schema {
                name: db_name.to_string(),
                tables: Some(HashMap::new()),
                functions: Some(HashMap::new()),
                procedures: Some(HashMap::new()),
                views: Some(HashMap::new()),
                constraints: Some(HashMap::new()),
                //  foreign_data_wrappers : Some(HashMap::new()),
                locks: Some(HashMap::new()),
                types: Some(HashMap::new()),
                triggers: Some(HashMap::new()),
                aggregates: Some(HashMap::new()),
                materalized_views: Some(HashMap::new()),
                catalogs: Some(HashMap::new()),
                sequences: Some(HashMap::new()),
                roles : Some(HashMap::new()),
                type_: Some("schema".to_string()),
            };

            let db_metadata = DatabaseMetadata {
                name: db_name.to_string(),
                schemas: Some(HashMap::new()),
                foreign_data_wrappers: None,
                catalogs: None,
                type_: "database".to_string(),
            };
            self.databases.insert(db_name.to_string(), db_metadata);
            log::info!("Database node created for {}", db_name);
        }
        Ok(())
    }

    ///Get the database by its name
    pub async fn get_database_(
        &self,
        db_name: &str,
    ) -> std::option::Option<dashmap::mapref::one::Ref<'_, std::string::String, DatabaseMetadata>>
    {
        self.databases.get(db_name)
    }

    ///connect to rbatis if it isnt cached
    async fn rbatis_connect(
        &self,
        db_name: &str,
    ) -> Result<Option<Ref<'_, String, Arc<rbatis::RBatis>>>, rbdc::Error> {
        let cached_rb = self.rb_map.get(db_name);
        if cached_rb.is_some() {
            log::info!("rb cached");
            return Ok(cached_rb);
        }
        log::info!("rb isnt cached");
        let url = String::from(format!(
            "{}://{}:{}@{}:{}/{}",
            self.connection_info.driver_type,
            self.connection_info.username,
            self.connection_info.password,
            self.connection_info.server,
            self.connection_info.port,
            db_name
        ));
        if let Err(err) = self.connect(db_name, &url).await {
            println!("Connection failed for database {}: {:?}", db_name, err);
            return Err(rbdc::Error::from("Failed to connect to database"));
        }
        match self.rb_map.get(db_name) {
            Some(rb) => {
                log::info!("Connection successfull, rb cached");
                Ok(Some(rb))
            }
            None => {
                log::error!("Connection failed to cache rbatis");
                Err(rbdc::Error::from(
                    "Database not found after connection attempt",
                ))
            }
        }
        //Ok(rb)
    }

    pub async fn init_database(&self) {
        let databases = self.get_databases().await.unwrap();
        let mut schemas: Vec<ValueMap> = Vec::new();
        let mut tables: Vec<ValueMap> = Vec::new();
        let mut columns: Vec<ValueMap> = Vec::new();
        for dab in databases {
            for db_name in dab.0 {
                let db_namae = db_name.1.as_str().unwrap();
                schemas = self.get_schemas(db_namae).await.unwrap();
                self.init_schemas(&schemas, db_namae).await;
                for schema in schemas {
                    for sch in schema.0 {
                        let schema_namae = sch.1.as_str().unwrap();
                        tables = self.get_tables(db_namae, schema_namae).await.unwrap();
                        self.init_tables(&tables, db_namae, schema_namae).await;
                        for table in tables {
                            for t in table.0 {
                                let table_namae = t.1.as_str().unwrap();
                                columns = self
                                    .get_columns(db_namae, table_namae, schema_namae)
                                    .await
                                    .unwrap();
                                self.init_columns(&columns, db_namae, schema_namae, table_namae)
                                    .await;
                                let ind = self
                                    .get_indexes(db_namae, table_namae, schema_namae)
                                    .await
                                    .unwrap();
                                self.init_indexes(&ind, db_namae, schema_namae, table_namae)
                                    .await;
                                let constr = self
                                    .get_constraints(db_namae, schema_namae, table_namae)
                                    .await
                                    .unwrap();
                                self.init_constraints(&constr, db_namae, schema_namae, table_namae)
                                    .await;
                                let functions =
                                    self.get_functions(db_namae, schema_namae).await.unwrap();
                                self.init_functions(&functions, db_namae).await;
                            }
                        }
                        let views = self.get_views(db_namae, schema_namae).await.unwrap();
                        self.init_views(&views, db_namae, schema_namae).await;

                        let stored_procs = self
                            .get_stored_procedures(db_namae, schema_namae)
                            .await
                            .unwrap();
                        self.init_stored_procedures(&stored_procs, db_namae, schema_namae)
                            .await;
                        let trigfuncs = self.get_trigger_functions(db_namae).await.unwrap();
                        self.init_trigger_functions(&trigfuncs, db_namae).await;
                        let seqs = self.get_sequences(db_namae, schema_namae).await.unwrap();
                        self.init_sequences(&seqs, db_namae, schema_namae).await;
                        let matv = self.get_materalized_views(db_namae).await.unwrap();
                        self.init_materalized_views(&matv, db_namae, schema_namae)
                            .await;
                        // let fdw = self.get_foreign_data_wrappers(db_namae).await.unwrap();
                        // self.init_foreign_data_wrappers(&fdw,db_namae).await;
                        let typez = self.get_types(db_namae).await.unwrap();
                        self.init_types(&typez, db_namae, schema_namae).await;
                    }
                }
            }
        }
    }

    async fn init_tables(&self, result2: &Vec<ValueMap>, db_name: &str, schema_name: &str) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            let table_map = node
                .schemas
                .as_mut()
                .unwrap()
                .get_mut(schema_name)
                .unwrap()
                .tables
                .get_or_insert_with(HashMap::new);
            for table in result2 {
                let table_name = table
                    .0
                    .get(&Value::String("table_name".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let tb_node = Table {
                    name: table_name.to_string(),
                    columns: Some(HashMap::new()),
                    constraints: Some(HashMap::new()),
                    indexes: Some(HashMap::new()),
                    triggers: Some(HashMap::new()),
                    rules: Some(HashMap::new()),
                    rls_policies: Some(HashMap::new()),
                    type_: Some("table".to_string()),
                    schema_name:Some(schema_name.to_string()),
                    db_name: db_name.to_string(),
                };
                table_map.insert(table_name.to_string(), tb_node);
            }
        }
    }

    async fn init_columns(
        &self,
        result2: &Vec<ValueMap>,
        db_name: &str,
        schema_name: &str,
        table_name: &str,
    ) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            if let Some(table_map) = node
                .schemas
                .as_mut()
                .unwrap()
                .get_mut(schema_name)
                .unwrap()
                .tables
                .as_mut()
                .unwrap()
                .get_mut(table_name)
            {
                let columns_map = table_map.columns.get_or_insert_with(HashMap::new);
                for col in result2 {
                    let col_name = col
                        .0
                        .get(&Value::String("column_name".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let data_type = col
                        .0
                        .get(&Value::String("data_type".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let is_nullable = col
                        .0
                        .get(&Value::String("is_nullable".to_string()))
                        .and_then(|v| v.as_bool())
                        .unwrap_or_default();

                    let _col_node = crate::domain::metadata::column::Column {
                        name: String::from(col_name), //col_name.1.to_string(),
                        data_type: Some(String::from(data_type)),
                        is_nullable: Some(is_nullable),
                        default_value: None,
                        is_primary_key: None,
                        maximum_length: None,
                        type_: "column".to_string(),
                        table_name: table_name.to_string(),
                        schema_name:Some(schema_name.to_string()),
                        db_name: db_name.to_string(),
                    };

                    columns_map.insert(col_name.to_string(), _col_node);
                }
            }
        }
    }

    async fn init_views(&self, result2: &Vec<ValueMap>, db_name: &str, schema_name: &str) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            let view_map = node
                .schemas
                .as_mut()
                .unwrap()
                .get_mut(schema_name)
                .unwrap()
                .views
                .get_or_insert_with(HashMap::new); //node.value_mut().views.get_or_insert_with(HashMap::new);
            for view in result2 {
                let view_name = view
                    .0
                    .get(&Value::String("view_name".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let view_definition = view
                    .0
                    .get(&Value::String("text".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let view_node = View {
                    name: view_name.to_string(),
                    definition: view_definition.to_string(),
                    type_: "view".to_string(),
                    schema_name: schema_name.to_string(),
                    db_name: db_name.to_string(),
                };
                view_map.insert(view_name.to_string(), view_node);
            }
        } else {
            log::info!("Node is not OK");
        }
    }

    async fn init_stored_procedures(
        &self,
        result2: &Vec<ValueMap>,
        db_name: &str,
        schema_name: &str,
    ) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            let stored_procedure_map = node
                .schemas
                .as_mut()
                .unwrap()
                .get_mut(schema_name)
                .unwrap()
                .procedures
                .get_or_insert_with(HashMap::new); //node.value_mut().procedures.get_or_insert_with(HashMap::new);
            for stored_procedure in result2 {
                let stored_procedure_name = stored_procedure
                    .0
                    .get(&Value::String("object_name".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let stored_procedure_definition = stored_procedure
                    .0
                    .get(&Value::String("text".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let stored_procedure_node = Procedure {
                    name: stored_procedure_name.to_string(),
                    definition: stored_procedure_definition.to_string(),
                    parameters: None,
                    source_db: crate::domain::datb::database_type::DatabaseType::Oracle,
                    type_: "procedure".to_string(),
                    schema_name:Some(schema_name.to_string()),
                    db_name: db_name.to_string(),
                };
                stored_procedure_map
                    .insert(stored_procedure_name.to_string(), stored_procedure_node);
            }
        }
    }

    async fn init_functions(&self, result2: &Vec<ValueMap>, db_name: &str) {
        //let mut db_struct = self.databases.lock().unwrap();
        if let Some(mut node) = self.databases.get_mut(db_name) {
            let function_map = node
                .value_mut()
                .schemas
                .as_mut()
                .unwrap()
                .get_mut("public")
                .unwrap()
                .functions
                .get_or_insert_with(HashMap::new);
            for function in result2 {
                let function_name = function
                    .0
                    .get(&Value::String("object_name".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let function_schema = function
                    .0
                    .get(&Value::String("object_type".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let function_node = Function {
                    name: function_name.to_string(),
                    definition: function_schema.to_string(),
                    parameters: None,
                    return_type: None,
                    type_: Some("function".to_string()),
                    schema_name:Some("public".to_string()),
                    db_name: db_name.to_string(),
                    full_function: None,
                };
            }
        } else {
            log::info!("Node is not OK");
        }
    }

    async fn init_trigger_functions(&self, result2: &Vec<ValueMap>, db_name: &str) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            let trigger_map = node
                .value_mut()
                .schemas
                .as_mut()
                .unwrap()
                .get_mut("public")
                .unwrap()
                .triggers
                .get_or_insert_with(HashMap::new);
            for trigger in result2 {
                let trigger_name = trigger
                    .0
                    .get(&Value::String("trigger_name".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let trigger_definition = trigger
                    .0
                    .get(&Value::String("trigger_definition".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let trigger_node = trigger::TriggerFunction {
                    name: trigger_name.to_string(),
                    definition : trigger_definition.to_string(),
                    type_: "trigger".to_string(),
                    db_name: db_name.to_string(),
                    schema_name: "public".to_string(),
                };
                trigger_map.insert(trigger_name.to_string(), trigger_node);
            }
        } else {
            log::info!("Node is not OK");
        }
    }

    async fn init_aggregates(&self, result2: &Vec<ValueMap>, db_name: &str) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            let aggregate_map = node
                .value_mut()
                .schemas
                .as_mut()
                .unwrap()
                .get_mut("public")
                .unwrap()
                .aggregates
                .get_or_insert_with(HashMap::new);
            for aggregate in result2 {
                let aggregate_name = aggregate
                    .0
                    .get(&Value::String("proname".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let aggregate_node = Aggregate {
                    name: aggregate_name.to_string(),
                };
                aggregate_map.insert(aggregate_name.to_string(), aggregate_node);
            }
        }
    }

    async fn init_materalized_views(
        &self,
        result2: &Vec<ValueMap>,
        db_name: &str,
        schema_name: &str,
    ) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            let matview_map = node
                .value_mut()
                .schemas
                .as_mut()
                .unwrap()
                .get_mut(schema_name)
                .unwrap()
                .materalized_views
                .get_or_insert_with(HashMap::new);
            for matview in result2 {
                let matview_name = matview
                    .0
                    .get(&Value::String("matviewname".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let matview_def = matview
                    .0
                    .get(&Value::String("definition".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let matview_node = MateralizedView {
                    name: matview_name.to_string(),
                    definition: matview_def.to_string(),
                };
                matview_map.insert(matview_name.to_string(), matview_node);
            }
        } else {
            log::info!("Node is not OK");
        }
    }

    async fn init_catalogs(&self, result2: &Vec<ValueMap>, db_name: &str) {
        // let mut db_struct = self.databases.lock().unwrap();
        if let Some(mut node) = self.databases.get_mut(db_name) {
            let catalog_map = node.value_mut().catalogs.get_or_insert_with(HashMap::new);
            for catalog in result2 {
                let catalog_name = catalog
                    .0
                    .get(&Value::String("catalog_name".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let catalog_node = Catalog {
                    name: catalog_name.to_string(),
                };

                catalog_map.insert(catalog_name.to_string(), catalog_node);
            }
        } else {
            log::info!("Node is not OK");
        }
    }

    async fn init_schemas(&self, result2: &Vec<ValueMap>, db_name: &str) {
        /*
        TODO
         if let Some(mut node) = self.databases.get_mut(db_name){
             let schemamap = node.value_mut().schemas.get_or_insert_with(HashMap::new);
             for schema in result2{
                 let schema_name = schema.0.get(&Value::String("schema_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();

                 let schema_node = crate::metadata::database::Schema{
                     name : schema_name.to_string(),
                     tables : Some(HashMap::new()),
                     functions : Some(HashMap::new()),
                     procedures : Some(HashMap::new()),
                     views : Some(HashMap::new()),
                     constraints : Some(HashMap::new()),
                     locks : Some(HashMap::new()),
                     types : Some(HashMap::new()),
                     triggers : Some(HashMap::new()),
                     aggregates : Some(HashMap::new()),
                     materalized_views : Some(HashMap::new()),
                     catalogs : Some(HashMap::new()),
                     sequences : Some(HashMap::new()),
                 };
                 schemamap.insert(schema_name.to_string(), schema_node);
             }
         }*/
    }

    async fn init_indexes(
        &self,
        result2: &Vec<ValueMap>,
        db_name: &str,
        schema_name: &str,
        table_name: &str,
    ) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            if let Some(table_map) = node
                .schemas
                .as_mut()
                .unwrap()
                .get_mut(schema_name)
                .unwrap()
                .tables
                .as_mut()
                .unwrap()
                .get_mut(table_name)
            {
                //node.value_mut().tables.clone().unwrap().get_mut(db_name){
                let index_map = table_map.indexes.get_or_insert_with(HashMap::new);
                for index in result2 {
                    let index_name = index
                        .0
                        .get(&Value::String("index_name".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let column_name: &str = index
                        .0
                        .get(&Value::String("column_name".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    if index_map.contains_key(index_name) {
                        index_map
                            .get(index_name)
                            .unwrap()
                            .column_name
                            .clone()
                            .unwrap()
                            .push(column_name.to_string());
                    } else {
                        let non_unique = index
                            .0
                            .get(&Value::String("non_unique".to_string()))
                            .and_then(|v| v.as_bool())
                            .unwrap_or_default();
                        let table_name = index
                            .0
                            .get(&Value::String("table_name".to_string()))
                            .and_then(|v| v.as_str())
                            .unwrap_or_default();

                        let index_node = crate::domain::metadata::index::Index {
                            name: String::from(index_name),
                            definition: None,
                            column_name: Some(Vec::new()),
                            non_unique: Some(non_unique),
                            table_name: Some(String::from(table_name)),
                            db_name: db_name.to_string(),
                            schema_name:Some(schema_name.to_string()),
                            type_:"index".to_string()
                        };
                        index_node
                            .column_name
                            .clone()
                            .unwrap()
                            .push(column_name.to_string());
                        index_map.insert(index_name.to_string(), index_node);
                    }
                }
            }
        } else {
            log::info!("Node is not OK");
        }
    }

    async fn init_constraints(
        &self,
        result2: &Vec<ValueMap>,
        db_name: &str,
        schema_name: &str,
        table_name: &str,
    ) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            if let Some(table_map) = node
                .schemas
                .as_mut()
                .unwrap()
                .get_mut(schema_name)
                .unwrap()
                .tables
                .as_mut()
                .unwrap()
                .get_mut(table_name)
            {
                //node.value_mut().tables.clone().unwrap().get_mut(table_name){
                let constraint_map = table_map.constraints.get_or_insert_with(HashMap::new);
                for constraint in result2 {
                    let constraint_name = constraint
                        .0
                        .get(&Value::String("constraint_name".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let constraint_type = constraint
                        .0
                        .get(&Value::String("constraint_type".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let table_name = constraint
                        .0
                        .get(&Value::String("table_name".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let column_name = constraint
                        .0
                        .get(&Value::String("column_name".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let constraint_node = crate::domain::metadata::constraint::Constraint {
                        name: String::from(constraint_name),
                        c_type: String::from(constraint_type),
                        table_name: String::from(table_name),
                        column_name: String::from(column_name),
                        db_name: db_name.to_string(),
                        schema_name:Some(schema_name.to_string()),
                        type_: "constraint".to_string(),
                        fk_table: todo!(),
                        fk_column: todo!(),
                    };
                    constraint_map.insert(constraint_name.to_string(), constraint_node);
                }
            }
        } else {
            log::info!("Node is not OK");
        }
    }

    async fn init_types(&self, result2: &Vec<ValueMap>, db_name: &str, schema_name: &str) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            let type_map = node
                .schemas
                .as_mut()
                .unwrap()
                .get_mut(schema_name)
                .unwrap()
                .types
                .get_or_insert_with(HashMap::new);
            for type_ in result2 {
                let type_name = type_
                    .0
                    .get(&Value::String("type_name".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let field_name = type_
                    .0
                    .get(&Value::String("attribute_name".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let field_type = type_
                    .0
                    .get(&Value::String("data_type".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                if let Some(existing_type) = type_map.get_mut(type_name) {
                    existing_type.fields.as_mut().unwrap().push(TypeField {
                        name: field_name.to_string(),
                        type_name: field_type.to_string(),
                    });
                } else {
                    let mut type_node = crate::domain::metadata::utype::Type {
                        name: type_name.to_string(),
                        fields: Some(Vec::new()),
                    };
                    type_node.fields.as_mut().unwrap().push(TypeField {
                        name: field_name.to_string(),
                        type_name: field_type.to_string(),
                    });
                    type_map.insert(type_name.to_string(), type_node);
                }
            }
        }
    }

    async fn init_sequences(&self, result2: &Vec<ValueMap>, db_name: &str, schema_name: &str) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            let sequence_map = node
                .schemas
                .as_mut()
                .unwrap()
                .get_mut(schema_name)
                .unwrap()
                .sequences
                .get_or_insert_with(HashMap::new); //node.value_mut().sequences.get_or_insert_with(HashMap::new);
            for seq in result2 {
                let sequence_name = seq
                    .0
                    .get(&Value::String("sequence_name".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let sequence_node = Sequence {
                    name: sequence_name.to_string(),
                    start_val: todo!(),
                    minimum_val: todo!(),
                    maximum_val: todo!(),
                    increment: todo!(),
                    cycle: todo!(),
                    type_: Some("sequence".to_string()),
                    schema_name:Some(schema_name.to_string()),
                    db_name: db_name.to_string(),
                };
                sequence_map.insert(sequence_name.to_string(), sequence_node);
            }
        } else {
            log::info!("Node is not OK");
        }
    }

    async fn init_rls_policies(
        &self,
        result2: &Vec<ValueMap>,
        db_name: &str,
        schema_name: &str,
        table_name: &str,
    ) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            if let Some(table_map) = node
                .schemas
                .as_mut()
                .unwrap()
                .get_mut(schema_name)
                .unwrap()
                .tables
                .as_mut()
                .unwrap()
                .get_mut(table_name)
            {
                //node.value_mut().tables.clone().unwrap().get_mut(table_name){
                let policy_map = table_map.rls_policies.get_or_insert_with(HashMap::new);
                for policy in result2 {
                    let policy_name = policy
                        .0
                        .get(&Value::String("policy_name".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let policy_node = RlsPolicy {
                        name: policy_name.to_string(),
                        command: policy
                            .0
                            .get(&Value::String("command".to_string()))
                            .and_then(|v| v.as_str())
                            .unwrap_or_default()
                            .to_string(),
                    };
                    policy_map.insert(policy_name.to_string(), policy_node);
                }
            }
        }
    }

    pub fn get_row_value<'a>(
        &'a self,
        table_name: &'a str,
        db_name: &'a str,
        columns: &'a Vec<String>,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send + 'a {
        async move {
            let rb = self.rbatis_connect(db_name).await?.unwrap();
            let _sql = format!("SELECT {} FROM {};", columns.join(","), table_name);
            let result: Vec<ValueMap> = rb.query_decode(&_sql, vec![]).await.unwrap();
            println!("row get result: {:?}", result);
            Ok(result)
        }
    }
}

impl DatabaseRepository for OracleRepository {
    async fn get_databases(&self) -> Result<Vec<ValueMap>, rbdc::Error> {
        log::info!("Oraclerepository: Get databases");
        let rb = self.rbatis_connect("postgres").await?.unwrap();
        let _sql = "SELECT username FROM all_users;";
        let result = rb.query(_sql, vec![]).await.unwrap();
        let result2: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

        //iterate through databases and insert into the pool and the database map(db.1 = database name)
        if let Some(databases) = result.as_array() {
            for db_val in databases {
                for db in db_val {
                    if !self.rb_map.contains_key(db.1.as_str().unwrap()) {
                        let rb = Arc::new(rbatis::RBatis::new());

                        self.rb_map.insert(db.1.to_string(), rb);

                        let db_node = Schema {
                            name: db.1.to_string(),
                            tables: Some(HashMap::new()),
                            functions: Some(HashMap::new()),
                            procedures: Some(HashMap::new()),
                            views: Some(HashMap::new()),
                            constraints: Some(HashMap::new()),
                            // foreign_data_wrappers : Some(HashMap::new()),
                            locks: Some(HashMap::new()),
                            types: Some(HashMap::new()),
                            triggers: Some(HashMap::new()),
                            aggregates: Some(HashMap::new()),
                            materalized_views: Some(HashMap::new()),
                            catalogs: Some(HashMap::new()),
                            sequences: Some(HashMap::new()),
                            roles:Some(HashMap::new()),
                            type_: Some("schema".to_string()),
                        };
                        let db_metadata = DatabaseMetadata {
                            name: db.1.to_string(),
                            schemas: Some(HashMap::new()),
                            foreign_data_wrappers: None,
                            catalogs: None,
                            type_: "database".to_string(),
                        };
                        self.databases.insert(db.1.to_string(), db_metadata);
                    }
                }
            }
        }
        Ok(result2)
    }

    ///Get all tables in the database
    async fn get_tables(
        &self,
        db_name: &str,
        schema_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();
        let _sql = "SELECT  table_name
                        FROM all_tables
                        WHERE owner = '?';";
        let result2: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(String::from(db_name))])
            .await
            .unwrap();
        if result2.is_empty() {
            return Ok(Vec::new());
        }
        Ok(result2)
    }

    ///Get all columns in the table
    async fn get_columns(
        &self,
        db_name: &str,
        table_name: &str,
        schema_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                        column_name,
                        data_type,
                        data_length,
                        nullable
                    FROM all_tab_columns
                    WHERE table_name = '?'
                    AND owner = '?';";

        let result2: Vec<ValueMap> = rb
            .query_decode(
                _sql,
                vec![
                    Value::String(table_name.to_string()),
                    Value::String(db_name.to_string()),
                ],
            )
            .await
            .unwrap();

        Ok(result2)
    }

    async fn get_views(
        &self,
        db_name: &str,
        schema_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                        view_name,
                        text
                        FROM all_views
                        WHERE owner = '?';";
        let result2: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(db_name.to_string())])
            .await
            .unwrap();

        Ok(result2)
    }

    async fn get_stored_procedures(
        &self,
        db_name: &str,
        schema_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                            p.owner AS schema_name,
                            p.object_name AS procedure_name,
                            LISTAGG(a.argument_name || ' ' || a.data_type, ', ') WITHIN GROUP (ORDER BY a.position) AS arguments,
                            p.procedure_name AS return_type,
                            p.object_type AS procedure_body
                        FROM 
                            all_procedures p
                        LEFT JOIN 
                            all_arguments a ON p.object_id = a.object_id 
                                        AND p.owner = a.owner
                                        AND p.procedure_name = a.procedure_name
                        WHERE 
                            p.owner = :? 
                            AND p.object_type = 'PROCEDURE'
                        GROUP BY 
                            p.owner, p.object_name, p.procedure_name, p.object_type;
                        ";
        let result2: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(String::from(db_name))])
            .await
            .unwrap();

        Ok(result2)
    }

    async fn get_functions(
        &self,
        db_name: &str,
        schema_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                            o.owner AS schema_name,
                            o.object_name AS function_name,
                            LISTAGG(a.argument_name || ' ' || a.data_type, ', ') WITHIN GROUP (ORDER BY a.position) AS arguments,
                            s.text AS function_body
                        FROM 
                            all_objects o
                        JOIN 
                            all_source s ON o.object_name = s.name AND o.owner = s.owner
                        LEFT JOIN 
                            all_arguments a ON a.object_name = o.object_name AND a.owner = o.owner
                        WHERE 
                            o.object_type = 'FUNCTION'
                            AND o.owner = ?
                        GROUP BY 
                            o.owner, o.object_name, s.text;";
        let result2: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(String::from(db_name))])
            .await
            .unwrap();
        Ok(result2)
    }

    async fn get_trigger_functions(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                            trigger_name,
                        FROM all_triggers
                        WHERE owner = '?';";
        let result2: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(String::from(db_name))])
            .await
            .unwrap();

        Ok(result2)
    }

    async fn get_event_triggers(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT evtname 
        FROM pg_event_trigger;";
        let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

        Ok(result)
    }

    async fn get_aggregates(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT proname 
        FROM pg_proc 
        WHERE prokind='a';";
        let result2: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

        Ok(result2)
    }

    async fn get_materalized_views(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();
        let _sql = "SELECT matviewname,definition
        FROM pg_matviews;";
        let result2: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
        Ok(result2)
    }

    ///Get the types from the database if exists
    async fn get_types(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT ot.object_type_name AS type_name,
                            oa.attribute_name,
                            oa.data_type
                        FROM user_objects o
                        JOIN user_types ot ON o.object_name = ot.type_name
                        JOIN user_type_attrs oa ON ot.type_name = oa.type_name
                        WHERE o.object_type = 'TYPE'
                        ORDER BY ot.object_type_name, oa.attribute_name;";
        let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
        Ok(result)
    }

    async fn get_languages(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let url = "postgresql://mzeteny:zetou123@localhost:5432/".to_string() + db_name;
        let _ = self.connect(db_name, url.as_str()).await;
        let rb = match self.rb_map.get(db_name) {
            Some(rb) => rb,
            None => return Err(rbdc::Error::from("database not found")),
        };
        let _sql = "SELECT lanname 
        FROM pg_language;";
        let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

        Ok(result)
    }

    async fn get_catalogs(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT nspname AS catalog_name
                        FROM pg_namespace
                        WHERE nspname IN ('pg_catalog', 'information_schema')
                        ORDER BY nspname;";
        let result2: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
        Ok(result2)
    }

    async fn get_foreign_data_wrappers(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        Ok(Vec::new())
    }

    //TODO SELECT table_name FROM information_schema.tables WHERE table_schema = '?'; catalogobjects(?)
    async fn get_schemas(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT schema_name 
        FROM information_schema.schemata
        WHERE schema_name='public';";
        let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
        Ok(result)
    }

    async fn get_indexes(
        &self,
        db_name: &str,
        table_name: &str,
        schema_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                        index_name,
                    FROM all_indexes
                    WHERE table_owner = '?';";
        let result2: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(String::from(db_name))])
            .await
            .unwrap();

        Ok(result2)
    }

    async fn get_constraints(
        &self,
        db_name: &str,
        table_name: &str,
        schema_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                            ucc.constraint_name,
                            ucc.table_name,
                            ucc.column_name,
                            uc.constraint_type
                        FROM 
                            user_cons_columns ucc
                        JOIN 
                            user_constraints uc 
                        ON 
                            ucc.constraint_name = uc.constraint_name
                        WHERE 
                            ucc.table_name = 'YOUR_TABLE_NAME';";
        let result2: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(String::from(db_name))])
            .await
            .unwrap();
        Ok(result2)
    }

    async fn get_sequences(
        &self,
        db_name: &str,
        schema_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                        sequence_name
                    FROM all_sequences
                    WHERE sequence_owner = 'YOUR_SCHEMA';";
        let result2: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(String::from(db_name))])
            .await
            .unwrap();

        Ok(result2)
    }

    async fn get_roles_and_users(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                        username
                    FROM all_users;";
        let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
        Ok(result)
    }

    async fn get_table_statistics(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                    table_name,
                    num_rows,
                    blocks,
                    empty_blocks,
                    avg_row_len
                FROM all_tables
                WHERE owner = 'YOUR_SCHEMA';";
        let result: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(String::from(db_name))])
            .await
            .unwrap();
        Ok(result)
    }

    async fn get_active_sessions(&self) -> Result<Vec<ValueMap>, rbdc::Error> {
        self.connect("postgres", self.base_url.as_str()).await;
        let rb = match self.rb_map.get("postgres") {
            Some(rb) => rb,
            None => return Err(rbdc::Error::from("database not found")),
        };
        let _sql = "SELECT 
                        sid,
                        serial#,
                        username,
                        status,
                        machine
                    FROM v$session;";
        let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
        Ok(result)
    }

    async fn get_locks(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();
        let _sql = "SELECT 
                    sid,
                    serial#,
                    type,
                    mode_held,
                    mode_requested
                FROM v$lock;";
        let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
        Ok(result)
    }

    async fn get_partitions(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                        partition_name,
                    FROM all_tab_partitions
                    WHERE table_owner = '?';";
        let result: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(table_name.to_string())])
            .await
            .unwrap();
        Ok(result)
    }

    async fn get_user_privileges(
        &self,
        db_name: &str,
        user_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                grantee,
                privilege,
                table_name
            FROM all_tab_privs
            WHERE owner = 'YOUR_SCHEMA';";
        let result: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(db_name.to_string())])
            .await
            .unwrap();
        Ok(result)
    }

    async fn get_database_settings(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                        parameter,
                        value
                    FROM v$parameter;";
        let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
        Ok(result)
    }

    async fn get_foreign_key_relationships(
        &self,
        db_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT conname AS constraint_name, conrelid::regclass AS table_name,
    a.attname AS column_name, confrelid::regclass AS foreign_table_name,
    af.attname AS foreign_column_name
        FROM   pg_constraint
        JOIN   pg_attribute a ON a.attnum = ANY(conkey) AND a.attrelid = conrelid
        JOIN   pg_attribute af ON af.attnum = ANY(confkey) AND af.attrelid = confrelid
        WHERE  contype = 'f';";
        let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
        Ok(result)
    }

    async fn get_triggers_associated_with_table(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT tgname
        FROM pg_trigger
        WHERE tgrelid = ?::regclass;";
        let result: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(table_name.to_string())])
            .await
            .unwrap();
        Ok(result)
    }

    async fn get_default_columns_value(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT column_name, column_default
        FROM information_schema.columns
        WHERE table_name = ?;";
        let result: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(table_name.to_string())])
            .await
            .unwrap();
        Ok(result)
    }

    async fn get_rls_policies(
        &self,
        db_name: &str,
        table_name: &str,
        schema_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT policy_name FROM DBA_POLICIES WHERE OBJECT_NAME = '?';";
        let result2: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(table_name.to_string())])
            .await
            .unwrap();
        Ok(result2)
    }

    async fn get_rules(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        Ok(Vec::new())
    }
}
