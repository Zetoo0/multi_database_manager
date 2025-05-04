use crate::domain::create_info::create_table_info::{CreateIndexInfo, CreateSequenceInfo, CreateTableInfo, CreateViewInfo};
use crate::domain::metadata::aggregate::Aggregate;
use crate::domain::metadata::catalog::Catalog;
use crate::domain::metadata::column::Column;
use crate::domain::metadata::constraint::Constraint;
use crate::domain::metadata::database::Schema;
use crate::domain::metadata::database_metadata::DatabaseMetadata;
use crate::domain::metadata::index::Index;
use crate::domain::metadata::rls_policy::RlsPolicy;
use crate::domain::metadata::role::{self, Role};
use crate::domain::metadata::rule::Rule;
use crate::domain::metadata::sequence::Sequence;
use crate::domain::metadata::trigger::Trigger;
use crate::domain::metadata::utype::TypeField;
use crate::domain::metadata::{
    function::Function, materalized_view::MateralizedView, procedure::Procedure, table::Table,
    trigger, view::View,
};
use crate::domain::repository::database_repository::DatabaseRepository;
use crate::DatabaseConnection;
use dashmap::mapref::one::Ref;
use dashmap::DashMap;
use rbdc_mssql::*;
use rbs::value::map::ValueMap;
use rbs::Value;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
#[derive(Clone)]
pub struct MsSqlRepository {
    pub rb_map: DashMap<String, Arc<rbatis::RBatis>>,
    base_url: String,
    pub databases: DashMap<String, DatabaseMetadata>,
    connection_info: DatabaseConnection,
}

impl MsSqlRepository {
    pub fn new(connection_info: DatabaseConnection) -> Self {
        let rb_map = DashMap::new();
        let databases = DashMap::new();
        let base_url = String::from(format!(
            "{}://{}:{}@{}:{}/master",
            connection_info.driver_type,
            connection_info.username,
            connection_info.password,
            connection_info.server,
            connection_info.port
        ));
        return MsSqlRepository {
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
            match rb.init(MssqlDriver {}, url) {
                Ok(_) => log::info!("Connection to {} successful", db_name),
                Err(e) => {
                    log::error!("Failed to initialize rbatis for {}: {:?}", db_name, e);
                    return Err(Box::new(e)); // Return the error early
                }
            }
            self.rb_map.insert(db_name.to_string(), rb);
            let db_metadata = DatabaseMetadata {
                name: db_name.to_string(),
                schemas: Some(HashMap::new()),
                foreign_data_wrappers: Some(HashMap::new()),
                catalogs: Some(HashMap::new()),
                type_: "database".to_string(),
            };
            /*let db_node = Database{
                name : db_name.to_string(),
                tables : Some(HashMap::new()),
                functions : Some(HashMap::new()),
                procedures : Some(HashMap::new()),
                views : Some(HashMap::new()),
                constraints : Some(HashMap::new()),
                foreign_data_wrappers : Some(HashMap::new()),
                locks : Some(HashMap::new()),
                types : Some(HashMap::new()),
                triggers : Some(HashMap::new()),
                aggregates : Some(HashMap::new()),
                materalized_views : Some(HashMap::new()),
                catalogs : Some(HashMap::new()),
                sequences : Some(HashMap::new()),
            };*/
            self.databases.insert(db_name.to_string(), db_metadata);
            log::info!("Database node created for {}", db_name);
        }
        Ok(())
    }

    pub async fn get_database_(
        &self,
        db_name: &str,
    ) -> std::option::Option<dashmap::mapref::one::Ref<'_, std::string::String, DatabaseMetadata>>
    {
        self.databases.get(db_name)
    }

    //connect to rbatis if it isnt cached
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
            log::error!(
                "{}",
                format!("Connection failed for database {}: {}", db_name, err)
            );
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
    }

    async fn get_schema_id(&self, db_name: &str, schema_name: &str) -> Value {
        let rb = self.rbatis_connect(db_name).await.unwrap();
        let _sql = "SELECT SCHEMA_ID(?)";
        let result: Value = rb
            .expect("Expected rb pool")
            .query_decode(_sql, vec![Value::String(schema_name.to_string())])
            .await
            .unwrap();
        return result;
    }

    async fn init_tables(&self, result2: &Vec<ValueMap>, db_name: &str, schema_name: &str) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            if let Some(schemas) = node
                .value_mut()
                .schemas
                .as_mut()
                .unwrap()
                .get_mut(schema_name)
            {
                let table_map = schemas.tables.get_or_insert_with(HashMap::new);
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
                        schema_name: Some(schema_name.to_string()),
                        db_name: db_name.to_string(),
                    };
                    table_map.insert(table_name.to_string(), tb_node);
                }
            }
        }
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
                                
                                // let ind = self.get_indexes(db_namae,table_namae,schema_namae).await.unwrap();
                                // self.init_indexes(&ind,db_namae,schema_namae,table_namae).await;
                                let constr = self
                                    .get_constraints(db_namae, schema_namae, table_namae)
                                    .await
                                    .unwrap();
                                self.init_constraints(&constr, db_namae, schema_namae, table_namae)
                                    .await;
                                let functions =
                                    self.get_functions(db_namae, schema_namae).await.unwrap();
                                self.init_functions(&functions, db_namae,schema_namae).await;
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
                        self.init_trigger_functions(&trigfuncs, db_namae, schema_namae).await;
                        let seqs = self.get_sequences(db_namae, schema_namae).await.unwrap();
                        self.init_sequences(&seqs, db_namae, schema_namae).await;
                        let matv = self.get_materalized_views(db_namae).await.unwrap();
                        self.init_materalized_views(&matv, db_namae, schema_namae)
                            .await;
                        //let fdw = self.get_foreign_data_wrappers(db_namae).await.unwrap();
                        //self.init_foreign_data_wrappers(&fdw,db_namae).await;
                        let typez = self.get_types(db_namae).await.unwrap();
                        self.init_types(&typez, db_namae, schema_namae).await;
                    }
                }
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
            if let Some(schemas) = node.schemas.as_mut().unwrap().get_mut(schema_name) {
                if let Some(table_map) = schemas.tables.as_mut().unwrap().get_mut(table_name) {
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
                        let column_default = col
                            .0
                            .get(&Value::String("column_default".to_string()))
                            .and_then(|v| v.as_str())
                            .unwrap_or_default();

                        let _col_node = crate::domain::metadata::column::Column {
                            name: String::from(col_name), //col_name.1.to_string(),
                            data_type: Some(String::from(data_type)),
                            is_nullable: Some(is_nullable),
                            default_value: Some(String::from(column_default)),
                            is_primary_key: None,
                            maximum_length: None,
                            type_: "column".to_string(),
                            table_name: table_name.to_string(),
                            db_name: db_name.to_string(),
                            schema_name: Some(schema_name.to_string())
                        };

                        columns_map.insert(col_name.to_string(), _col_node);
                    }
                }
            }
        }
    }

    async fn init_views(&self, result2: &Vec<ValueMap>, db_name: &str, schema_name: &str) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            if let Some(schemas) = node.schemas.as_mut().unwrap().get_mut(schema_name) {
                let view_map = schemas.views.get_or_insert_with(HashMap::new); //node.value_mut().views.get_or_insert_with(HashMap::new);
                for view in result2 {
                    let view_name = view
                        .0
                        .get(&Value::String("TABLE_NAME".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let view_definition = view
                        .0
                        .get(&Value::String("VIEW_DEFINITION".to_string()))
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
            if let Some(schemas) = node.schemas.as_mut().unwrap().get_mut(schema_name) {
                let stored_procedure_map = schemas.procedures.get_or_insert_with(HashMap::new); //node.value_mut().procedures.get_or_insert_with(HashMap::new);
                for stored_procedure in result2 {
                    let stored_procedure_name = stored_procedure
                        .0
                        .get(&Value::String("procedure_name".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let stored_procedure_definition = stored_procedure
                        .0
                        .get(&Value::String("procedure_body".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let params = stored_procedure
                        .0
                        .get(&Value::String("PARAMETER_NAME".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let mut stored_procedure_node = Procedure {
                        name: stored_procedure_name.to_string(),
                        definition: stored_procedure_definition.to_string(),
                        parameters: Some(Vec::new()),
                        source_db: crate::domain::datb::database_type::DatabaseType::Postgres,
                        type_: "procedure".to_string(),
                        schema_name: Some(schema_name.to_string()),
                        db_name: db_name.to_string(),
                    };
                    stored_procedure_node.parameters = Some(
                        params
                            .split(',')
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>(),
                    );
                    stored_procedure_map
                        .insert(stored_procedure_name.to_string(), stored_procedure_node);
                }
            }
        }
    }


    //TODO
    async fn init_functions(&self, result2: &Vec<ValueMap>, db_name: &str, schema_name: &str) {
        //let mut db_struct = self.databases.lock().unwrap();
        if let Some(mut node) = self.databases.get_mut(db_name) {
            let function_map = node
                .value_mut()
                .schemas
                .clone()
                .unwrap()
                .get_mut("public")
                .unwrap()
                .functions
                .get_or_insert_with(HashMap::new);
            for function in result2 {
                let function_name = function
                    .0
                    .get(&Value::String("name".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let function_schema = function
                    .0
                    .get(&Value::String("type_desc".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let function_node = Function {
                    name: function_name.to_string(),
                    definition: function_schema.to_string(),
                    parameters: Some(Vec::new()),
                    return_type: None,
                    type_: Some("function".to_string()),
                    schema_name: Some(schema_name.to_string()),
                    db_name: db_name.to_string(),
                    full_function: None
                };
            }
        } else {
            log::info!("Node is not OK");
        }
    }

    async fn init_trigger_functions(&self, result2: &Vec<ValueMap>, db_name: &str,schema_name: &str) {
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
                    .get(&Value::String("name".to_string()))
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
                    schema_name: schema_name.to_string(),
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
                    .get(&Value::String("name".to_string()))
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
                    .get(&Value::String("name".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let matview_def = matview
                    .0
                    .get(&Value::String("type_desc".to_string()))
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
                    .get(&Value::String("name".to_string()))
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
        if let Some(mut node) = self.databases.get_mut(db_name) {
            let schemamap = node.value_mut().schemas.get_or_insert_with(HashMap::new);
            for schema in result2 {
                let schema_name = schema
                    .0
                    .get(&Value::String("name".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();

                let schema_node = crate::domain::metadata::database::Schema {
                    name: schema_name.to_string(),
                    tables: Some(HashMap::new()),
                    functions: Some(HashMap::new()),
                    procedures: Some(HashMap::new()),
                    views: Some(HashMap::new()),
                    constraints: Some(HashMap::new()),
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
                schemamap.insert(schema_name.to_string(), schema_node);
            }
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
            if let Some(schemas) = node
                .value_mut()
                .schemas
                .clone()
                .unwrap()
                .get_mut(schema_name)
            {
                if let Some(table_map) = schemas.tables.as_mut().unwrap().get_mut(table_name) {
                    //node.value_mut().tables.clone().unwrap().get_mut(table_name){
                    let constraint_map = table_map.constraints.get_or_insert_with(HashMap::new);
                    for constraint in result2 {
                        let constraint_name = constraint
                            .0
                            .get(&Value::String("CONSTRAINT_NAME".to_string()))
                            .and_then(|v| v.as_str())
                            .unwrap_or_default();
                        let constraint_type = constraint
                            .0
                            .get(&Value::String("CONSTRAINT_TYPE".to_string()))
                            .and_then(|v| v.as_str())
                            .unwrap_or_default();
                        let table_name = constraint
                            .0
                            .get(&Value::String("TABLE_NAME".to_string()))
                            .and_then(|v| v.as_str())
                            .unwrap_or_default();
                        let column_name = constraint
                            .0
                            .get(&Value::String("COLUMN_NAME".to_string()))
                            .and_then(|v| v.as_str())
                            .unwrap_or_default();

                        let constraint_node = crate::domain::metadata::constraint::Constraint {
                            name: String::from(constraint_name),
                            c_type: String::from(constraint_type),
                            table_name: String::from(table_name),
                            column_name: String::from(column_name),
                            db_name: db_name.to_string(),
                            schema_name: Some(schema_name.to_string()),
                            type_:"constraint".to_string(),
                            fk_table: todo!(),
                            fk_column: todo!(),
                        };
                        constraint_map.insert(constraint_name.to_string(), constraint_node);
                    }
                }
            }
        } else {
            log::info!("Node is not OK");
        }
    }

    async fn init_sequences(&self, result2: &Vec<ValueMap>, db_name: &str, schema_name: &str) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            if let Some(schemas) = node.schemas.as_mut().unwrap().get_mut(schema_name) {
                let sequence_map = schemas.sequences.get_or_insert_with(HashMap::new); //node.value_mut().sequences.get_or_insert_with(HashMap::new);
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
                        db_name: db_name.to_string(),
                        schema_name: Some(schema_name.to_string()),
                        type_: Some("sequence".to_string()),
                    };
                    sequence_map.insert(sequence_name.to_string(), sequence_node);
                }
            }
        } else {
            log::info!("Node is not OK");
        }
    }

    async fn init_types(&self, result2: &Vec<ValueMap>, db_name: &str, schema_name: &str) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            if let Some(schemas) = node
                .value_mut()
                .schemas
                .as_mut()
                .unwrap()
                .get_mut(schema_name)
            {
                let type_map = schemas.types.get_or_insert_with(HashMap::new);
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
    }

    async fn init_rls_policies(
        &self,
        result2: &Vec<ValueMap>,
        db_name: &str,
        schema_name: &str,
        table_name: &str,
    ) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            if let Some(schema_map) = node
                .value_mut()
                .schemas
                .clone()
                .unwrap()
                .get_mut(schema_name)
            {
                if let Some(table_map) = schema_map.tables.as_mut().unwrap().get_mut(table_name) {
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
        } else {
            log::info!("Node is not OK");
        }
    }

    async fn init_rules(&self, result2: &Vec<ValueMap>, db_name: &str, schema_name: &str) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            if let Some(table_map) = node
                .value_mut()
                .schemas
                .as_mut()
                .unwrap()
                .get_mut(schema_name)
                .unwrap()
                .tables
                .clone()
                .unwrap()
                .get_mut(db_name)
            {
                let rule_map = table_map.rules.get_or_insert_with(HashMap::new); //table_map.unwrap().get(table_name).unwrap().rules.get_or_insert_with(HashMap::new);//table_map.as_mut().unwrap().get(table_name).unwrap().rules.get_or_insert_with(HashMap::new());
                for rule in result2 {
                    let rule_name = rule
                        .0
                        .get(&Value::String("rule_name".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let rule_node = Rule {
                        name: rule_name.to_string(),
                        definition: rule
                            .0
                            .get(&Value::String("o.object_id".to_string()))
                            .and_then(|v| v.as_str())
                            .unwrap_or_default()
                            .to_string(),
                    };
                    rule_map.insert(rule_name.to_string(), rule_node);
                }
            }
        } else {
            log::info!("Node is not OK");
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
    pub fn add_column<'a>(
        &'a self,
        table_name: &'a str,
        col: Column,
        database_name: &'a str,
    ) -> impl Future<Output = Result<Column, ()>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(database_name) {
                let mut pk = String::new();
                let mut is_nullable = String::new();
                if col.is_primary_key.unwrap() {
                    pk = "PRIMARY KEY".to_string();
                } else if col.maximum_length != Some(0) {
                    pk = format!("({:?})", col.maximum_length.unwrap());
                } else {
                    pk = "".to_string();
                }
                if !col.is_nullable.unwrap() {
                    is_nullable = "NOT NULL".to_string();
                } else {
                    is_nullable = "".to_string();
                }

                let _sql = format!(
                    "ALTER TABLE {} ADD COLUMN {} {} {} {};",
                    table_name,
                    col.name,
                    col.data_type.as_ref().unwrap(),
                    pk,
                    is_nullable
                );
                println!("sql: {:?}", &_sql);
                let result: Value = rb.query_decode(&_sql, vec![]).await.unwrap();
                self.databases
                    .get_mut(database_name)
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut("public")
                    .unwrap()
                    .tables
                    .as_mut()
                    .unwrap()
                    .get_mut(table_name)
                    .unwrap()
                    .columns
                    .as_mut()
                    .unwrap()
                    .insert(col.name.clone(), col.clone());
                Ok(col)
                //Ok(col)
            } else {
                Err(())
            }
        }
    }
    pub fn create_table<'a>(
        &'a self,
        table_info: &'a CreateTableInfo,
    ) -> impl Future<Output = Result<Table, rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(table_info.db_name.as_str()) {
                let mut col_transformed_datas: Vec<String> = Vec::new();
                let mut pk = String::new();
                let mut is_nullable = String::new();
                let mut col_map: HashMap<String, Column> = HashMap::new();
                for col in &table_info.columns {
                    if col.is_primary_key.unwrap() {
                        pk = "PRIMARY KEY".to_string();
                    } else if col.maximum_length != Some(0) {
                        pk = format!("({:?})", col.maximum_length.unwrap());
                    } else {
                        pk = "".to_string();
                    }
                    if !col.is_nullable.unwrap() {
                        is_nullable = "NOT NULL".to_string();
                    } else {
                        is_nullable = "".to_string();
                    }
                    col_transformed_datas.push(format!(
                        "{} {} {} {}",
                        col.name,
                        col.data_type.as_ref().unwrap(),
                        pk,
                        is_nullable
                    ));
                    col_map.insert(col.name.clone(), col.clone());
                }
                let _sql = format!(
                    "CREATE TABLE {}({});",
                    table_info.table_name,
                    col_transformed_datas.join(",")
                );
                println!("sql: {:?}", &_sql);
                let result: Value = rb.query_decode(&_sql, vec![]).await.unwrap();
                let table_name_clone = table_info.table_name.clone();
                let new_table = Table {
                    name: table_name_clone,
                    columns: Some(col_map),
                    constraints: None,
                    indexes: None,
                    triggers: None,
                    rules: None,
                    rls_policies: None,
                    type_: Some("table".to_string()),
                    schema_name: Some(table_info.schema_name.clone()),
                    db_name: table_info.table_name.clone(),
                };
                let table_clone = new_table.clone();
                self.databases
                    .get_mut(&table_info.db_name)
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut(&table_info.schema_name)
                    .unwrap()
                    .tables
                    .as_mut()
                    .unwrap()
                    .insert(table_info.table_name.clone(), table_clone);
                Ok(new_table)
            } else {
                Err(rbdc::Error::from("database not found"))
            }
        }
    }

    pub fn create_sequence<'a>(
        &'a self,
        table_name: &'a str,
        database_name: &'a str,
        schema_name: &'a str,
        create_seq_info: &'a CreateSequenceInfo,
    ) -> impl Future<Output = Result<Sequence, rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(database_name) {
                let mut _sql = String::new();
                if create_seq_info.cycle {
                    _sql = format!("CREATE SEQUENCE {} INCREMENT BY {} MINVALUE {} MAXVALUE {} START WITH {} CYCLE;",create_seq_info.sequence_name,create_seq_info.increment,create_seq_info.minimum_val,create_seq_info.maximum_val,create_seq_info.start_val);
                } else {
                    _sql = format!("CREATE SEQUENCE {} INCREMENT BY {} MINVALUE {} MAXVALUE {} START WITH {} NOCYCLE;",create_seq_info.sequence_name,create_seq_info.increment,create_seq_info.minimum_val,create_seq_info.maximum_val,create_seq_info.start_val);
                }
                let result: Value = rb.query_decode(&_sql, vec![]).await.unwrap();
                let sequence: Sequence = Sequence {
                    name: create_seq_info.sequence_name.clone(),
                    start_val: Some(create_seq_info.start_val.clone()),
                    increment: Some(create_seq_info.increment.clone()),
                    cycle: Some("CYCLE".to_string()),
                    minimum_val: Some(create_seq_info.minimum_val.clone()),
                    maximum_val: Some(create_seq_info.maximum_val.clone()),
                    type_: Some("sequence".to_string()),
                    schema_name: Some(schema_name.to_string()),
                    db_name: database_name.to_string(),
                };
                self.databases
                    .get_mut(database_name)
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut(table_name)
                    .unwrap()
                    .sequences
                    .as_mut()
                    .unwrap()
                    .insert(create_seq_info.sequence_name.clone(), sequence.clone());
                Ok(sequence)
            } else {
                Err(rbdc::Error::from("database not found"))
            }
        }
    }

    pub async fn create_funtion<'a>(
        &'a self,
        database_name: &'a str,
        create_function: Function,
    ) -> impl Future<Output = Result<Function, rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(database_name) {
                let mut _sql = format!(
                    "CREATE FUNCTION {}({:?})\n{};END;  ",
                    create_function.name, create_function.parameters, create_function.definition
                );
                let result_: Result<(), rbdc::Error> = rb.query_decode(&_sql, vec![]).await.unwrap();

                self.databases
                    .get_mut(database_name)
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut(create_function.schema_name.as_ref().clone().unwrap())
                    .unwrap()
                    .functions
                    .as_mut()
                    .unwrap()
                    .insert(create_function.name.clone(), create_function.clone());
                log::info!("Result: {:?}", result_);
                let function_created = create_function.clone();
                Ok(function_created)
            } else {
                Err(rbdc::Error::from("database not found!"))
            }
        }
    }

    pub fn create_index<'a>(
        &'a self,
        index_info: &'a CreateIndexInfo,
        database_name: &'a str,
    ) -> impl Future<Output = Result<Index, rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(database_name) {
                let mut _sql = String::new();
                /*  if index_info..unwrap(){
                    _sql = format!("CREATE UNIQUE INDEX {} ON {} ({});",index_info.index_name,index_info.table_name,index_info.columns.join(","));
                }else{*/
                _sql = format!(
                    "CREATE INDEX {} ON {} ({});",
                    index_info.index_name,
                    index_info.table_name,
                    index_info.columns.join(",")
                );
                //}
                let result_:Result<(), rbdc::Error>  = rb.query_decode(&_sql, vec![]).await;
                let index: Index = Index {
                    name: index_info.index_name.clone(),
                    definition: todo!(),
                    column_name: Some(index_info.columns.clone()),
                    non_unique: todo!(),
                    table_name: Some(index_info.table_name.clone()),
                    db_name: database_name.to_string(),
                    schema_name: Some(index_info.schema_name.to_string()),
                    type_: "index".to_string(),
                };
                self.databases.get_mut(database_name)
                .unwrap()
                .schemas.as_mut().unwrap()
                .get_mut(&index_info.schema_name)
                .unwrap()
                .tables
                .unwrap()
                .get_mut(&index_info.table_name)
                .unwrap()
                .indexes
                .as_mut()
                .unwrap()
                .insert(index_info.index_name, index.clone());
                log::info!("Result: {:?}", &result_);
                Ok(index)
            // .unwrap()
            } else {
                Err(rbdc::Error::from("database not found"))
            }
        }
    }

    pub fn create_view<'a>(
        &'a self,
        view_info: &'a CreateViewInfo,
        database_name: &'a str,
    ) -> impl Future<Output = Result<View, rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(database_name) {
                let mut _sql = String::new();
                _sql = format!("CREATE VIEW {} AS {}", view_info.view_name, view_info.stmt);
                let result:Result<(), rbdc::Error> = rb.query_decode(&_sql, vec![]).await;
                let view: View = View {
                    db_name: database_name.to_string(),
                    name: view_info.view_name.clone(),
                    definition: view_info.stmt.clone(),
                    type_: "view".to_string(),
                    schema_name: "public".to_string(),
                };
                log::info!("Result: {:?}", &result);
                self.databases
                    .get_mut(database_name)
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut(&view_info.schema_name)
                    .unwrap()
                    .views
                    .as_mut()
                    .unwrap()
                    .insert(view_info.view_name.clone(), view.clone());
                Ok(view)
            } else {
                Err(rbdc::Error::from("database not found"))
            }
        }
    }

    pub async fn create_role<'a>(
        &'a self,
        role_info : Role,
        /*role_name: &'a str,
        password: Option<&'a str>,
        can_login: bool,
        is_superuser: bool,
        can_create_db: bool,
        can_create_role: bool,
        connection_limit: Option<i32>,
        valid_until: Option<&'a str>,*/
        database_name: &'a str
    ) -> impl Future<Output = Result<Role, rbdc::Error>> + Send + 'a {
        async move{
            if let Some(rb) = self.rb_map.get(database_name){
                let mut sql = format!("CREATE ROLE {} WITH {} {} {} {};",
                    role_info.name,
                    if role_info.can_login.unwrap() { "LOGIN" } else { "NOLOGIN" },
                    if role_info.is_super.unwrap() { "SUPERUSER" } else { "NOSUPERUSER" },
                    if role_info.is_create_db.unwrap() { "CREATEDB" } else { "NOCREATEDB" },
                    if role_info.is_create_role.unwrap() { "CREATEROLE" } else { "NOCREATEROLE" },
                   /* x if role_info.valid_until.is_some() {
                        format!("VALID UNTIL '{}'", role_info.valid_until.unwrap())
                    } else {
                        "".to_string()
                    }*/
                );
               /*  if let Some(pw) = role_info.password {
                    sql.push_str(&format!(" PASSWORD '{}'", pw));
                }
                if let Some(limit) = role_info.connection_limit {
                    sql.push_str(&format!(" CONNECTION LIMIT {}", limit));
                } */       
                log::info!("sql: {}", sql);
                let result:Result<(), rbdc::Error> = rb.query_decode(&sql, vec![]).await;
                let role: Role = Role {
                    name: role_info.name.clone(),
                    is_super: role_info.is_super,
                    is_insherit: role_info.is_insherit,
                    is_create_role: role_info.is_create_role,
                    is_create_db: role_info.is_create_db,
                    can_login: role_info.can_login,
                    is_replication: role_info.is_replication,
                    connection_limit: role_info.connection_limit,
                    valid_until: role_info.valid_until,
                    password: role_info.password,
                    db_name: database_name.to_string(),
                    type_: "role".to_string(),
                    schema_name: role_info.schema_name,
                };
                log::info!("Result: {:?}", &result);
                self.databases
                        .get_mut(database_name)
                        .unwrap()
                        .schemas
                        .as_mut()
                        .unwrap()
                        .get_mut(&role.schema_name)
                        .unwrap()
                        .roles
                        .as_mut()
                        .unwrap()
                        .insert(role_info.name.to_string(), role.clone());

                Ok(role)
                /*if result.is_ok(){
                    log::info!("create role result: {:?}", result);
                    Ok(())
                }else{
                    log::info!("create role error: {:?}", result);
                    result
                }*/
            }else{
                Err(rbdc::Error::from("database not found"))
            }
        }
       
    }

    pub fn create_schema<'a>(
        &'a self,
        schema_create_name: &'a str,//Search for the schema info
        database_name: &'a str,
        user_name: Option<&'a str>,
    ) -> impl Future<Output = Result<Schema, rbdc::Error>> + Send + 'a {
        async move{
            if let Some(rb) = self.rb_map.get(database_name){
                let sql = if let Some(user) = user_name {
                    format!("CREATE SCHEMA {} AUTHORIZATION {}", schema_create_name, user)
                } else {
                    format!("CREATE SCHEMA IF NOT EXISTS {}", schema_create_name)
                };
            
                
                let result:Result<(), rbdc::Error> = rb.query_decode(&sql, vec![]).await;
                let schema = Schema {
                    name: schema_create_name.to_string(),
                    type_: Some("schema".to_string()),
                    roles: Some(HashMap::new()),
                    tables: Some(HashMap::new()),
                    views: Some(HashMap::new()),
                    sequences: Some(HashMap::new()),
                    functions: Some(HashMap::new()),
                    procedures: Some(HashMap::new()),
                    constraints: Some(HashMap::new()),
                    locks: Some(HashMap::new()),
                    triggers: Some(HashMap::new()),
                    types: Some(HashMap::new()),
                    aggregates: Some(HashMap::new()),
                    materalized_views: todo!(),
                    catalogs: todo!(),
                };
                log::info!("Result: {:?}", &result);
                Ok(schema)
                /*if result.is_ok(){
                    log::info!("create schema result: {:?}", result);
                    Ok(())
                }else{
                    log::info!("create schema error: {:?}", result);
                    result
                }*/
            }else{
                Err(rbdc::Error::from("database not found"))
            }
        }
    } 

    pub fn create_trigger<'a>(
        &'a self,
        name:&'a str,
        when:&'a str,
        type_:&'a str,
        table_name:&'a str,
        function_name:&'a str,
        database_name: &'a str,
        schema_name: &'a str
    ) -> impl Future<Output = Result<Trigger,rbdc::Error>> + Send + 'a {
        async move{
            if let Some(rb) = self.rb_map.get(database_name){
                let _sql = format!("
                    CREATE TRIGGER {}
                        {} {}
                        ON {}
                       --FOR EACH ROW
                        EXECUTE PROCEDURE {}();
                ",name,when,type_,table_name,function_name);
                log::info!("sql: {}", _sql);
                let result_: Result<(), rbdc::Error> = rb.query_decode(&_sql, vec![]).await;
                let trigger: Trigger = Trigger{
                    name: name.to_string(),
                    definition: todo!(),
                    type_: todo!(),
                    db_name: database_name.to_string(),
                    schema_name: todo!(),
                };
                 self.databases
                    .get_mut(database_name)
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut(schema_name)
                    .unwrap()
                    .tables
                    .as_mut()
                    .unwrap()
                    .get_mut(table_name)
                    .as_mut()
                    .unwrap()
                    .triggers
                    .as_mut()
                    .unwrap()
                    .insert(name.to_string(), trigger);
                log::info!("Resutl: {:?}", &result_);
                Ok(trigger)
            }else{
                Err(rbdc::Error::from("database not found"))
            }

        }
    }

    pub async fn alter_table_column<'a>(
        &'a self,
        table_name: &'a str,
        new_col: Column,
        old_col: Column,
        database_name: &'a str,
    ) -> impl Future<Output = Result<Column, rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(database_name) {
                if old_col == new_col {
                    log::info!("It is the same reality!");
                    return Err(rbdc::Error::from("It is the same reality!"));
                } else {
                    let new_col_clone = new_col.clone();
                    let null_constraint =
                        if new_col.is_nullable.unwrap() && !old_col.is_nullable.unwrap() {
                            "DROP NOT"
                        } else if !new_col.is_nullable.unwrap() && old_col.is_nullable.unwrap() {
                            "SET"
                        } else {
                            ""
                        };
                    let primary_key_constraint = if new_col.is_primary_key.unwrap() {
                        format!(
                            "ADD CONSTRAINT {}_pk PRIMARY KEY ({})",
                            new_col.name, new_col.name
                        )
                    } else {
                        format!("DROP CONSTRAINT {}_pk", old_col.name)
                    };

                    let col_rename = if new_col.name != old_col.name {
                        format!(
                            "EXEC sp_rename {}.{}, {}, 'COLUMN';",
                            table_name, old_col.name, new_col.name
                        )
                    } else {
                        "".to_string()
                    };
                    let new_type = new_col.data_type.clone().unwrap();
                    let col_type = if new_col.data_type.unwrap() != old_col.data_type.unwrap() {
                        format!(
                            "ALTER TABLE {} ALTER COLUMN {} {};",
                            table_name, old_col.name, new_type
                        )
                    } else {
                        "".to_string()
                    };
                    let col_max_length = if (new_col.maximum_length.unwrap()
                        != old_col.maximum_length.unwrap())
                        && (new_col.maximum_length.unwrap() != 0)
                    {
                        format!(
                            "ALTER TABLE {} ALTER COLUMN {} VARCHAR({});",
                            table_name,
                            old_col.name,
                            new_col.maximum_length.unwrap()
                        )
                    } else {
                        "".to_string()
                    };
                    let new_default = new_col.default_value.clone().unwrap();
                    let col_default =
                        if new_col.default_value.unwrap() != old_col.default_value.unwrap() {
                            format!(
                                "ALTER TABLE {} ADD CONSTRAINT df_table_column DEFAULT {} FOR {};",
                                table_name, new_default, old_col.name
                            )
                        } else {
                            "".to_string()
                        };
                    let col_nullable =
                        if new_col.is_nullable.unwrap() != old_col.is_nullable.unwrap() {
                            format!(
                                "ALTER TABLE {} ALTER COLUMN {} {} NULL;",
                                table_name, new_col.name, null_constraint
                            )
                        } else {
                            "".to_string()
                        };
                    let col_pk =
                        if new_col.is_primary_key.unwrap() != old_col.is_primary_key.unwrap() {
                            format!("ALTER TABLE {} {};", table_name, primary_key_constraint)
                        } else {
                            "".to_string()
                        };
                    let alter2_sql = format!(
                        "
                        BEGIN;

                        -- Rename the column
                        {}
                        -- Change the data type
                        {}
                        -- Set the maximum length (for applicable data types like VARCHAR)
                        {}
                        -- Add or remove NOT NULL constraint
                        {}
                        
                        -- Set a default value
                        {}
                        -- Add or drop the primary key constraint
                        {}
                        COMMIT;
                        ",
                        col_rename, col_type, col_max_length, col_nullable, col_default, col_pk
                    );
                    log::info!("SQL SQL: {}", alter2_sql);
                    let result: Value = rb.query_decode(&alter2_sql, vec![]).await.unwrap();
                    log::info!("result: {}", result);
                    *self
                        .databases
                        .get_mut(database_name)
                        .unwrap()
                        .schemas
                        .as_mut()
                        .unwrap()
                        .get_mut(&old_col.schema_name.unwrap())
                        .unwrap()
                        .tables
                        .as_mut()
                        .unwrap()
                        .get_mut(table_name)
                        .unwrap()
                        .columns
                        .as_mut()
                        .unwrap()
                        .get_mut(old_col.name.as_str())
                        .unwrap() = new_col_clone.clone();
                    Ok(new_col_clone)
                    //Ok(new_table)
                }
            }else{
                return Err(rbdc::Error::from("database not found"));
            }
        }
    }

    pub async fn edit_constraint<'a>(
        &'a self,
        table_name: &'a str,
        new_constraint: Constraint,
        old_constraint: Constraint,
        database_name: &'a str,
    ) -> impl Future<Output = Result<Constraint, rbdc::Error>> + Send + 'a {
        async move{
            if let Some(rb) = self.rb_map.get(database_name){
                log::info!("DATABASE NAME: {:?}",database_name);
                let drop_sql = format!("ALTER TABLE {} DROP CONSTRAINT {};",table_name,old_constraint.name);
                let create_sql:String = match new_constraint.c_type.as_str() {
                    "FOREIGN KEY" => {
                        format!("ALTER TABLE {} ADD CONSTRAINT {} {} ({});",table_name,new_constraint.name,new_constraint.c_type,new_constraint.column_name)
                    },
                    _ => {
                        format!("ALTER TABLE {} ADD CONSTRAINT {} {} ({});",table_name,new_constraint.name,new_constraint.c_type,new_constraint.column_name)
                    }
                };
                
               // let create_sql = format!("ALTER TABLE {} ADD CONSTRAINT {} {} ({});",table_name,new_constraint.name,new_constraint.c_type,new_constraint.column_name);
                log::info!("drop_sql: {:?}",drop_sql);
                let drop_result:Result<(),rbdc::Error> = rb.query_decode(&drop_sql, vec![/*table_name.into(),old_constraint.name.into()*/]).await;
                log::info!("Drop result was ok? {:?}",drop_result);
               // if drop_result.is_ok(){
                    log::info!("create sql: {:?}", create_sql);
                    let create_result:Result<(), rbdc::Error> = rb.query_decode(&create_sql, vec![/*Value::String(table_name.to_string()), Value::String(new_constraint.name),Value::String(new_constraint.column_name)*/]).await;
                log::info!("Create result was ok? {:?}",create_result);
                let constraint_clone = new_constraint.clone();
                self.databases
                    .get_mut(database_name)
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut(new_constraint.schema_name.unwrap().as_str())
                    .unwrap()
                    .tables
                    .as_mut()
                    .unwrap()
                    .get_mut(new_constraint.table_name.as_str())
                    .unwrap()
                    .constraints
                    .as_mut()
                    .unwrap()
                    .insert(new_constraint.name.clone(), constraint_clone.clone());

                Ok(constraint_clone)

                //}else{
                  //  Ok(())
                
            }else{
                Err(rbdc::Error::from("database not found"))
            }
        }
    }
    
     
    /*pub async fn edit_role<'a>(
         &'a self,
        new_role: Role,
        old_role: Role,
        database_name: &'a str,
    ) -> impl Future<Output = Result<(), rbdc::Error>> + Send + 'a {
        async move{
            if let Some(rb) = self.rb_map.get(database_name){
                if(new_role != old_role){
                    let sql = format!("
                        ALTER ROLE {}
                    
                    ")
                }
        }
    }*/


    /*pub async fn edit_schema_info<'a>(//??? 
        &'a self,
       new_role: Role,
       old_role: Role,
       database_name: &'a str,
   ) -> impl Future<Output = Result<(), rbdc::Error>> + Send + 'a {
       todo!()
   }*/


    pub async fn edit_index<'a>(
        &'a self,
        table_name: &'a str,
        new_index: Index,
        old_index: Index,
        database_name: &'a str,
    ) -> impl Future<Output = Result<Index, rbdc::Error>> + Send + 'a {
        async move{
            if let Some(rb) = self.rb_map.get(database_name){
                let new_index_clone = new_index.clone();
                let drop_sql = format!("DROP INDEX {} ON {:?};",old_index.name,old_index.table_name.unwrap());
                let is_unique = if new_index.non_unique.unwrap(){"UNIQUE"}else{""};
                let create_sql = format!("CREATE INDEX {} ON {:?} ({:?})",new_index.name,new_index.table_name.unwrap(),new_index.column_name.unwrap().join(""));
                let drop_result:Result<(), rbdc::Error> = rb.query_decode(&drop_sql, vec![/*Value::String(old_index.name)*/]).await;
                //if drop_result.is_ok(){
                let create_result:Result<(), rbdc::Error> = rb.query_decode(&create_sql, vec![/*Value::String(new_index.name),Value::String(new_index.table_name.unwrap()),Value::String("valami".to_string())*/]). await;
                let mut database = self.databases.get_mut(database_name).unwrap();
                let indexes = database
                        .schemas
                        .as_mut()
                        .unwrap()
                        .get_mut(&old_index.schema_name.unwrap())
                        .unwrap()
                        .tables
                        .as_mut()
                        .unwrap()
                        .get_mut(table_name)
                        .unwrap()
                        .indexes
                        .as_mut()
                        .unwrap();
                    let new_index_clone_cl = new_index_clone.clone();
                    indexes.entry(old_index.name.clone()).or_insert(new_index_clone);
                    //behelyezés
                    Ok(new_index_clone_cl)
                    //create_result
                //}
                //else{
                  //  Ok(())
                
            }else{
                Err(rbdc::Error::from("database not found"))
            }
        }   
    }

    /*pub async fn edit_table_column<'a>(&'a self,table_name:&'a str, col:Column,database_name:&'a str)->impl Future<Output = Result<Column,()> > + Send + 'a{
        async move{
            if let Some(rb) = self.rb_map.get(database_name){
                let mut pk = String::new();
                let mut is_nullable = String::new();
                if col.is_primary_key.unwrap(){
                    pk = "PRIMARY KEY".to_string();
                }else if col.maximum_length != Some(0){
                    pk = format!("({:?})",col.maximum_length.unwrap());
                }else{
                    pk = "".to_string();
                }
                if !col.is_nullable.unwrap(){
                    is_nullable = "NOT NULL".to_string();
                }else{
                    is_nullable = "".to_string();
                }

                let _sql = format!("ALTER TABLE {} ALTER COLUMN {} TYPE {} {} {};",table_name,col.name,col.data_type.as_ref().unwrap(),pk,is_nullable);
                println!("sql: {:?}",&_sql);
                let result:Value = rb.query_decode(&_sql,vec![]).await.unwrap();
                Ok(col)
            }else{
                Err(())
            }
        }
    }*/

    pub async fn edit_sequence<'a>(
        &'a self,
        old_sequence: Sequence,
        new_sequence: Sequence,
        database_name: &'a str,
    ) -> impl Future<Output = Result<Sequence, rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(database_name) {
                let new_seq_clone = new_sequence.clone();
                let new_name = if new_sequence.name != old_sequence.name {
                    format!(
                        "ALTER SEQUENCE {} RENAME TO {};",
                        old_sequence.name, new_sequence.name
                    )
                } else {
                    "".to_string()
                };
                let new_increment = if new_sequence.increment != old_sequence.increment {
                    format!(
                        "ALTER SEQUENCE {} INCREMENT BY {};",
                        old_sequence.name,
                        new_sequence.increment.unwrap()
                    )
                } else {
                    "".to_string()
                };
                let new_max = if new_sequence.maximum_val != old_sequence.maximum_val {
                    format!(
                        "ALTER SEQUENCE {} MAXVALUE {};",
                        old_sequence.name,
                        new_sequence.maximum_val.unwrap()
                    )
                } else {
                    "".to_string()
                };
                let new_min = if new_sequence.minimum_val != old_sequence.minimum_val {
                    format!(
                        "ALTER SEQUENCE {} MINVALUE {};",
                        old_sequence.name,
                        new_sequence.minimum_val.unwrap()
                    )
                } else {
                    "".to_string()
                };
                let new_start = if new_sequence.start_val != old_sequence.start_val {
                    format!(
                        "ALTER SEQUENCE {} START WITH {};",
                        old_sequence.name,
                        new_sequence.start_val.unwrap()
                    )
                } else {
                    "".to_string()
                };
               /*  let new_cycle = if new_sequence.cycle != old_sequence.cycle {
                    format!("ALTER SEQUENCE {} NOCYCLE;", old_sequence.name)
                } else {
                    "".to_string()
                };*/

                let alter2_sql = format!(
                    "
                    BEGIN;

                    -- Rename the sequence
                    {}
                    -- Change the increment value
                    {}
                    -- Set the maximum value
                    {}
                    -- Set the minimum value
                    {}
                    -- Set the start value
                    {}
                    -- Add or drop the CYCLE constraint
                    
                    COMMIT;
                    ",
                    new_name, new_increment, new_max, new_min, new_start
                );
                log::info!("SQL SQL: {}", alter2_sql);
                let result: Result<(), rbdc::Error> = rb.query_decode(&alter2_sql, vec![]).await;
                *self
                    .databases
                    .get_mut(database_name)
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut(&old_sequence.schema_name.unwrap())
                    .unwrap()
                    .sequences
                    .as_mut()
                    .unwrap()
                    .get_mut(old_sequence.name.as_str())
                    .unwrap() = new_seq_clone.clone();
               // result
                Ok(new_seq_clone)
            } else {
                Err(rbdc::Error::from("Failed to connect to database"))
            }
        }
    }

     
    pub async fn edit_view<'a>(
        &'a self,
        old_view: View,
        new_view: View,
        database_name: &'a str,
    ) -> impl Future<Output = Result<View,rbdc::Error>> + Send + 'a {
        async move{
            if let Some(rb) = self.rb_map.get(database_name){
                let create_sql = format!("CREATE OR ALTER VIEW {} AS {};",old_view.name,new_view.definition);
                let create_or_replace_result:Result<(),rbdc::Error> = rb.query_decode(&create_sql, vec![]).await;
                let view:View = View{
                    name: new_view.name,
                    definition: new_view.definition,
                    type_: new_view.type_,
                    db_name: new_view.db_name,
                    schema_name: new_view.schema_name                    
                };
                self.databases
                .get_mut(database_name)
                .unwrap()
                .schemas
                .as_mut()
                .unwrap()
                .get_mut(&old_view.schema_name)
                .unwrap()
                .views
                .as_mut()
                .unwrap()
                .insert(view.name.clone(), view.clone());
                Ok(view)
                //create_or_replace_result
            }else{
                Err(rbdc::Error::from("Failed to connect to database"))
            }
        } 
    }
/*
    pub async fn edit_trigger<'a>(
        &'a self,
        old_trigger: Trigger,
        table_name:&'a str,
        new_triger: Trigger,
        database_name: &'a str,
    ) -> impl Future<Output = Result<Function, ()>> + Send + 'a {
        todo!()
    }

    pub async fn edit_trigger_func<'a>(
        &'a self,
        old_trigger_func: TriggerFunction,
        new_triger_func: TriggerFunction,
        database_name: &'a str,
    ) -> impl Future<Output = Result<Function, ()>> + Send + 'a {
        todo!()
    }*/

    pub async fn edit_function<'a>(
        &'a self,
        old_function: Function,
        new_function: Function,
        database_name: &'a str,
    ) -> impl Future<Output = Result<Function, rbdc::Error>> + Send + 'a {
        async move {
            // [2025-01-10T13:42:27.279837 INFO  writepad_lib::domain::repository::postgres_repository] Current function info: {String("function_body"): String("\nSELECT CASE\n  WHEN $2 IS NULL THEN $1\n  WHEN $1 IS NULL THEN $2\n  ELSE $1 || ', ' || $2\nEND\n"), String("return_type"): String("text"), String("arguments"): String("text, text"), String("function_name"): String("_group_concat"), String("schema_name"): String("public")}
            if let Some(rb) = self.rb_map.get(database_name) {
                if old_function != new_function {
                    let new_func_clone = new_function.clone();
                    let drop_sql = format!("IF OBJECT_ID('dbo.{}', 'FN') IS NOT NULL DROP FUNCTION {}; GO", old_function.name,old_function.name);
                    let result_drop: Value = rb.query_decode(&drop_sql, vec![]).await.unwrap();
                    let create_sql = format!(
                        "CREATE FUNCTION {}({:?})\n{}; GO",
                        new_function.name, new_function.parameters, new_function.definition
                    );
                    let result_create: Result<(), rbdc::Error> = rb.query_decode(&create_sql, vec![]).await;
                    *self
                        .databases
                        .get_mut(database_name)
                        .unwrap()
                        .schemas
                        .as_mut()
                        .unwrap()
                        .get_mut(&old_function.schema_name.unwrap())
                        .unwrap()
                        .functions
                        .as_mut()
                        .unwrap()
                        .get_mut(old_function.name.as_str())
                        .unwrap() = new_func_clone.clone();
                    //result_create
                    Ok(new_func_clone)
                } else {
                    Err(rbdc::Error::from("No changes detected"))
                }
            } else {
                Err(rbdc::Error::from("Database not found"))
            }
        }
    }

    pub async fn delete_table_column<'a>(
        &'a self,
        column_name: &'a str,
        table_name: &'a str,
        db_name: &'a str,
    ) -> impl Future<Output = Result<(), rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(db_name) {
                let sql = format!("ALTER TABLE {} DROP COLUMN {};", table_name, column_name);
                let result_del: Result<(), rbdc::Error> = rb.query_decode(&sql, vec![]).await;
                if result_del.is_ok(){
                    Ok(())
                }else{
                    result_del
                }
            } else {
                Ok(())
            }
        }
    }

    pub async fn base_delete<'a>(
        &'a self,
        delete_to_name: &'a str,
        object_name: &'a str,
        db_name: &'a str,
    ) -> impl Future<Output = Result<(), rbdc::Error>> + Send + 'a {
        async move {
            log::info!(
                "REPOSITORY delete_to_name: {:?}\nobject_name: {:?}\ndb_name: {:?}",
                delete_to_name,
                object_name,
                db_name
            );
            if let Some(rb) = self.rb_map.get(db_name) {
                let sql = format!("DROP {} IF EXISTS {};", delete_to_name, object_name);
                log::info!("Deleting base sql: {}", sql);
                let result: Result<(), rbdc::Error> = rb.query_decode(&sql, vec![]).await;
                if result.is_ok() {
                    log::info!("DELETE RESULT: {:?}", result);
                    Ok(())
                } else {
                    log::info!("Deleting base error: {:?}", result);
                    result
                }
                /*if let Some(result) = rb.query_decode(&sql,vec![]).await.ok(){
                    log::info!("DELETE RESULT: {:?}",result);
                    result
                }else{
                    log::info!("Deleting base error:");
                    Err(())
                }*/
            } else {
                log::error!("Database connection not found for db_name: {}", db_name);
                Ok(())
            }
        }
    }
}

impl DatabaseRepository for MsSqlRepository {
    ///Get all databases
    async fn get_databases(&self) -> Result<Vec<ValueMap>, rbdc::Error> {
        log::info!("PgRepository: Get databases");
        let rb = self.rbatis_connect("master").await?.unwrap();
        let _sql = "SELECT name FROM sys.databases;";
        let result2: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

        //iterate through databases and insert into the pool and the database map(db.1 = database name)
        /*if let Some(databases) = result.as_array(){
            for db_val in databases{
                for db in db_val{
                    if !self.rb_map.contains_key(db.1.as_str().unwrap()){
                        let rb = Arc::new(rbatis::RBatis::new());

                        self.rb_map.insert(db.1.to_string(),rb);

                        let db_node = Database{
                            name : db.1.to_string(),
                            tables : Some(HashMap::new()),
                            functions : Some(HashMap::new()),
                            procedures : Some(HashMap::new()),
                            views : Some(HashMap::new()),
                            constraints : Some(HashMap::new()),
                            foreign_data_wrappers : Some(HashMap::new()),
                            locks : Some(HashMap::new()),
                            types : Some(HashMap::new()),
                            triggers : Some(HashMap::new()),
                            aggregates : Some(HashMap::new()),
                            materalized_views : Some(HashMap::new()),
                            catalogs : Some(HashMap::new()),
                            sequences : Some(HashMap::new()),
                        };
                        self.databases.insert(db.1.to_string(), db_node);
                    }
                }
            }
        }*/

        Ok(result2)
    }

    ///Get all tables in the database
    async fn get_tables(
        &self,
        db_name: &str,
        schema_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();
        let _sql = "SELECT TABLE_NAME
                        FROM INFORMATION_SCHEMA.TABLES
                        WHERE TABLE_TYPE = 'BASE TABLE'
                        AND TABLE_SCHEMA = ?;";
        let result2: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(schema_name.to_string())])
            .await
            .unwrap();

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

        let _sql = "SELECT COLUMN_NAME, DATA_TYPE
                        FROM INFORMATION_SCHEMA.COLUMNS
                        WHERE TABLE_NAME = ?;";

        let result2: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(table_name.to_string())])
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

        let _sql = "SELECT TABLE_NAME,VIEW_DEFINITION    
                    FROM INFORMATION_SCHEMA.VIEWS
                    WHERE TABLE_SCHEMA = ?;";
        let result2: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(schema_name.to_string())])
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
        let schema_id = self.get_schema_id(db_name, schema_name).await;
        let _sql = "SELECT
                            s.name AS schema_name,
                            p.name AS procedure_name,
                            PARAMETER_NAME = STUFF((
                                SELECT ',' + parameter_name + ' ' + data_type 
                                FROM information_schema.parameters
                                WHERE specific_name = p.name AND specific_schema = s.name
                                FOR XML PATH(''), TYPE).value('.', 'NVARCHAR(MAX)'), 1, 1, ''),
                            p.type_desc AS return_type,
                            m.definition AS procedure_body
                        FROM 
                            sys.procedures p
                        JOIN 
                            sys.schemas s ON p.schema_id = s.schema_id
                        JOIN 
                            sys.sql_modules m ON p.object_id = m.object_id
                        WHERE 
                            s.name = @schema_name;";
        let result2: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(schema_id.to_string())])
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

        let _sql = format!(
            "SELECT 
                            SCHEMA_NAME(o.schema_id) AS schema_name,
                            o.name AS function_name,
                            p.parameter_id,
                            p.name AS parameter_name,
                            TYPE_NAME(p.user_type_id) AS data_type,
                            m.definition AS function_body
                        FROM 
                            sys.objects o
                        JOIN 
                            sys.sql_modules m ON m.object_id = o.object_id
                        LEFT JOIN 
                            sys.parameters p ON p.object_id = o.object_id
                        WHERE 
                            o.type = 'FN'  -- For scalar functions
                            OR o.type = 'IF'  -- For inline table-valued functions
                            OR o.type = 'TF'  -- For multi-statement table-valued functions
                        AND 
                            SCHEMA_NAME(o.schema_id) = {};",
            schema_name
        );
        let result2: Vec<ValueMap> = rb.query_decode(_sql.as_str(), vec![]).await.unwrap();

        Ok(result2)
    }

    async fn get_trigger_functions(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();
        let _sql = "SELECT name, type_desc
                            FROM sys.objects
                            WHERE type = 'TR';";
        let result2: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

        Ok(result2)
    }

    async fn get_event_triggers(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT name, type_desc
                            FROM sys.triggers
                            WHERE is_ms_shipped = 0 AND parent_class_desc = 'DATABASE_DDL_TRIGGER';";
        let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

        Ok(result)
    }

    async fn get_aggregates(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT name
                        FROM sys.objects
                        WHERE type = 'AF';";
        let result2: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

        Ok(result2)
    }

    async fn get_materalized_views(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT name, type_desc
                        FROM sys.views
                        WHERE is_ms_shipped = 0;";
        let result2: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

        Ok(result2)
    }

    async fn get_types(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT tt.name AS type_name, 
                            c.name AS attribute_name, 
                            TYPE_NAME(c.user_type_id) AS data_type
                        FROM sys.table_types tt
                        JOIN sys.columns c ON c.object_id = tt.type_table_object_id
                        JOIN sys.schemas s ON tt.schema_id = s.schema_id
                        WHERE s.name NOT IN ('sys')  -- Exclude system schemas
                        ORDER BY tt.name, c.column_id;";
        let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
        Ok(result)
    }

    async fn get_languages(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        Ok(Vec::new())
    }

    async fn get_catalogs(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT name
                            FROM sys.databases;";
        let result2: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
        Ok(result2)
    }

    async fn get_foreign_data_wrappers(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        Ok(Vec::new())
    }

    //TODO SELECT table_name FROM information_schema.tables WHERE table_schema = '?'; catalogobjects(?)

    async fn get_schemas(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT name
                        FROM sys.schemas;";
        let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

        Ok(result)
    }

    async fn get_indexes(
        &self,
        db_name: &str,
        table_name: &str,
        schema_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        Ok(Vec::new())
    }

    async fn get_constraints(
        &self,
        db_name: &str,
        table_name: &str,
        schema_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                            kcu.TABLE_NAME,
                            kcu.CONSTRAINT_NAME,
                            kcu.COLUMN_NAME,
                            tc.CONSTRAINT_TYPE
                        FROM 
                            INFORMATION_SCHEMA.KEY_COLUMN_USAGE kcu
                        JOIN 
                            INFORMATION_SCHEMA.TABLE_CONSTRAINTS tc 
                        ON 
                            kcu.CONSTRAINT_NAME = tc.CONSTRAINT_NAME
                        WHERE 
                            tc.TABLE_NAME = ?;";
        let result2: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(schema_name.to_string())])
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
                            s.name AS sequence_name, 
                            o.type_desc, 
                            sch.name AS schema_name
                        FROM sys.objects o
                        JOIN sys.schemas sch ON o.schema_id = sch.schema_id
                        WHERE o.type = 'SQ'
                        AND sch.name = ?;";
        let result2: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(schema_name.to_string())])
            .await
            .unwrap();

        Ok(result2)
    }

    async fn get_roles_and_users(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        Ok(Vec::new())
    }

    async fn get_table_statistics(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        Ok(Vec::new())
    }

    async fn get_active_sessions(&self) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect("mssql").await?.unwrap();
        let _sql = "SELECT session_id, login_name 
                        FROM sys.dm_exec_sessions;";
        let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
        Ok(result)
    }

    async fn get_locks(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT * FROM sys.dm_tran_locks;";
        let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
        Ok(result)
    }

    //TODO
    async fn get_partitions(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT * 
                        FROM sys.partitions 
                        WHERE object_id = OBJECT_ID('your_table_name');";
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

        let _sql = "SELECT grantee, privilege_type, table_name 
        FROM information_schema.role_table_grants 
        WHERE grantee = ?;";
        let result: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(user_name.to_string())])
            .await
            .unwrap();
        Ok(result)
    }

    async fn get_database_settings(&self, db_name: &str) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT * FROM sys.configurations;";
        let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
        Ok(result)
    }

    async fn get_foreign_key_relationships(
        &self,
        db_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        Ok(Vec::new())
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

        let _sql = "SELECT COLUMN_NAME, COLUMN_DEFAULT
                        FROM INFORMATION_SCHEMA.COLUMNS
                        WHERE TABLE_NAME = ?;";
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

        let _sql = "SELECT 
                            sp.name AS policy_name,
                            sp.object_id AS policy_id,
                            sps.name AS table_name,
                            sps.schema_id AS schema_id,
                            spr.predicate_definition,
                            spr.type_desc AS predicate_type,
                            spr.is_enabled
                        FROM 
                            sys.security_policies sp
                        JOIN 
                            sys.security_predicates spr ON sp.object_id = spr.object_id
                        JOIN 
                            sys.tables sps ON spr.target_object_id = sps.object_id
                        JOIN 
                            sys.schemas sch ON sps.schema_id = sch.schema_id  -- Join to get schema info
                        WHERE 
                            sp.is_enabled = 1
                            ABD sps.name = ?
                            AND sch.name = ?;";
        let result2: Vec<ValueMap> = rb
            .query_decode(
                _sql,
                vec![
                    Value::String(table_name.to_string()),
                    Value::String(schema_name.to_string()),
                ],
            )
            .await
            .unwrap();

        Ok(result2)
    }

    async fn get_rules(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> Result<Vec<ValueMap>, rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                        o.name AS rule_name,
                        o.object_id,
                        c.name AS column_name,
                        t.name AS table_name
                    FROM 
                        sys.objects o
                    JOIN 
                        sys.columns c
                        ON o.object_id = c.default_object_id
                    JOIN 
                        sys.tables t
                        ON c.object_id = t.object_id
                    WHERE 
                        o.type = 'R' -- 'R' for rule
                        AND t.name = ?;";
        let result2: Vec<ValueMap> = rb
            .query_decode(_sql, vec![Value::String(table_name.to_string())])
            .await
            .unwrap();
        Ok(result2)
    }
}


#[cfg(test)]
mod tests {
    use std::result;

    use crate::domain::metadata::database::Schema;
    use crate::domain::metadata::sequence::Sequence;

    use super::*;
    use mockall::*;
    use mockall::predicate::*;
    use rbatis::dark_std::sync::vec;
    use tokio::test;
    use rbatis::rbatis::RBatis;
    use crate::domain::repository::postgres_repository::PostgresRepository;
    
    
    /// ✅ **Helper function to setup test database connection**
    async fn setup_test_db() -> MsSqlRepository {
       // let rb = rbatis::RBatis::new();
        //rb.init(rbdc_pg::driver::PgDriver {}, "postgres://test_user:test_password@localhost/test_db")
          //  .unwrap();
        let connection_info:DatabaseConnection = DatabaseConnection{
            driver_type: "mssql".to_string(),
            username: "mzeteny".to_string(),
            password: "zetou123".to_string(),
            server: "localhost".to_string(),
            port: "1433".to_string(),
        };
        let url = String::from(format!(
            "{}://{}:{}@{}:{}/{}",
            connection_info.driver_type,
            connection_info.username,
            connection_info.password,
            connection_info.server,
            connection_info.port,
            "test_db"
        ));
        let rb = Arc::new(rbatis::rbatis::RBatis::new());
        rb.init(MssqlDriver {}, url.as_str());
        let repo = MsSqlRepository::new(connection_info);
        repo.rb_map.insert("test_db".to_string(),rb.clone());
        repo
    }

    #[tokio::test]
    async fn test_create_table_is_ok() {
        let repo = setup_test_db().await;
        // ✅ **Step 1: Define the table schema**   
      //  let test_table = CreateTableInfo {table_name:"test_table".to_string(),columns:vec![Column{name:"id".to_string(),data_type:Some("SERIAL".to_string()),is_primary_key:Some(true),is_nullable:Some(false),default_value:None,maximum_length:None, table_name: todo!(), db_name: todo!(), type_: todo!() },Column{name:"name".to_string(),data_type:Some("VARCHAR(255)".to_string()),is_primary_key:Some(false),is_nullable:Some(false),default_value:None,maximum_length:Some(255), table_name: todo!(), db_name: todo!(), type_: todo!() },], db_name: todo!() 
      //  };
    let table_t: CreateTableInfo = CreateTableInfo{table_name:"fasz".to_string(),columns:vec![Column{name:"namajo".to_string(),data_type:Some("INT".to_string()),is_nullable:Some(true),default_value:Some("".to_string()),is_primary_key:Some(false),maximum_length:Some(0),table_name:"test_t".to_string(),db_name:"test_db".to_string(),type_:"column".to_string(), schema_name: todo!() }],db_name:"test_db".to_string(), schema_name: "public".to_string() };

        println!("Test table: {:?}",table_t);
        let result = repo.create_table(&table_t).await;
        println!("Is error? {}",result.is_err());
        assert!(result.is_ok(), "Table creation should succeed");

        // ✅ **Step 4: Check if the table exists in the database**
  //      let check_query = "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'tablaeetest');";
    //    let exists: (bool,) = repo.rb_map.get("test_db").unwrap().query_decode(check_query, vec![]).await.unwrap();
    //    assert!(exists.0, "Table should exist in the database");

        // ✅ **Step 5: Cleanup (Drop the test table)**
        let drop_query = "DROP TABLE IF EXISTS fasz;";
        repo.rb_map.get("test_db").unwrap().exec(drop_query, vec![]).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_table_database_not_exists(){
        let repo = setup_test_db().await;

        let table_t: CreateTableInfo = CreateTableInfo{table_name:"fasz".to_string(),columns: vec![Column{name:"namajo".to_string(),data_type:Some("INT".to_string()),is_nullable:Some(true),default_value:Some("".to_string()),is_primary_key:Some(false),maximum_length:Some(0),table_name:"test_t".to_string(),db_name:"test_db".to_string(),type_:"column".to_string(), schema_name: todo!() }],db_name:"akela".to_string(), schema_name: todo!() };
        
        let result = repo.create_table(&table_t).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_sequence_is_ok() {
        let repo = setup_test_db().await;

        let table_t: CreateTableInfo = CreateTableInfo{table_name:"faszaa".to_string(),columns: vec![Column{name:"namajo".to_string(),data_type:Some("INT".to_string()),is_nullable:Some(true),default_value:Some("".to_string()),is_primary_key:Some(false),maximum_length:Some(0),table_name:"test_t".to_string(),db_name:"test_db".to_string(),type_:"column".to_string(), schema_name: todo!() }],db_name:"test_db".to_string(),schema_name: todo!()};
        let seq_info = CreateSequenceInfo {
            sequence_name: "test_sequa".to_string(),
            increment: "1".to_string(),
            minimum_val: "1".to_string(),
            maximum_val: "100".to_string(),
            start_val: "1".to_string(),
            cycle: true,
            schema_name: todo!()
        };
        let _ = repo.create_table(&table_t).await;  
        let result = repo.create_sequence("faszaa", "test_db","test_db", &seq_info).await;
        assert!(result.is_ok());

        let drop_query_t = "DROP TABLE IF EXISTS faszaa;";
        repo.rb_map.get("test_db").unwrap().exec(drop_query_t, vec![]).await.unwrap();
        let drop_query = "DROP SEQUENCE IF EXISTS test_sequa;";
        repo.rb_map.get("test_db").unwrap().exec(drop_query, vec![]).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_sequence_table_not_exist(){
        let repo = setup_test_db().await;
        let seq_info = CreateSequenceInfo {sequence_name:"test_sequa".to_string(),increment:"1".to_string(),minimum_val:"1".to_string(),maximum_val:"100".to_string(), start_val: "100".to_string(), cycle: false,schema_name:todo!() };

        let result = repo.create_sequence("akela", "test_db","public", &seq_info).await;
        assert!(result.is_err());
    }

   /*  #[test]
    async fn test_create_function() {
    
        let mut databases = DashMap::new();
        let mut schemas = HashMap::new();
        let mut functions = HashMap::new();
        schemas.insert("public".to_string(), Schema {functions:Some(functions), name: todo!(), procedures: todo!(), tables: todo!(), views: todo!(), constraints: todo!(), locks: todo!(), triggers: todo!(), types: todo!(), aggregates: todo!(), materalized_views: todo!(), catalogs: todo!(), sequences: todo!(), roles: todo!(), type_: todo!() });
        databases.insert("huwa".to_string(), DatabaseMetadata {schemas:Some(schemas), name: todo!(), foreign_data_wrappers: todo!(), catalogs: todo!(), type_: todo!() });

        let repo = PostgresRepository::new(DatabaseConnection {
            driver_type: "postgres".to_string(),
            username: "user".to_string(),
            password: "pass".to_string(),
            server: "localhost".to_string(),
            port: 5432,
        });

        let func = Function {name:"test_func".to_string(),parameters:vec!["text".to_string(),"text".to_string()],definition:"SELECT $1 || $2;".to_string(), return_type: todo!(), type_: todo!(), db_name: todo!() 
        };

        let result = repo.create_function("test_db", func).await;
        assert!(result.is_ok());
    }*/
    #[tokio::test]
    async fn test_create_index_is_ok() {
        let repo = setup_test_db().await;
        let table_t: CreateTableInfo = CreateTableInfo{table_name:"test_tabletete".to_string(),columns:vec![Column{name:"id".to_string(),data_type:Some("INT".to_string()),is_nullable:Some(true),default_value:Some("".to_string()),is_primary_key:Some(false),maximum_length:Some(0),table_name:"test_t".to_string(),db_name:"test_db".to_string(),type_:"column".to_string(), schema_name: todo!() }],db_name:"test_db".to_string(), schema_name: todo!() };

        let index_info = CreateIndexInfo {
            index_name: "test_idxq".to_string(),
            table_name: "test_tabletete".to_string(),
            columns: vec!["id".to_string()],
            schema_name: todo!(),
        };

        let _ = repo.create_table(&table_t).await;  
        let result = repo.create_index(&index_info, "test_db").await;
        
        println!("Is error? {}",result.is_err());
        println!("Result: {:?}",result);
        
        assert!(result.is_ok());

        let drop_query_t = "DROP TABLE IF EXISTS test_tabletete;";
        repo.rb_map.get("test_db").unwrap().exec(drop_query_t, vec![]).await.unwrap();
        let drop_query = "DROP INDEX IF EXISTS test_idxq;";
        repo.rb_map.get("test_db").unwrap().exec(drop_query, vec![]).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_index_err_table_not_exist(){
        let repo = setup_test_db().await;
        let index_info = CreateIndexInfo {
            index_name: "test_idxq".to_string(),
            table_name: "test_tabletete".to_string(),
            columns: vec!["id".to_string()],
            schema_name: todo!(),
        };
        let result = repo.create_index(&index_info, "test_db").await;
        assert!(result.is_err());
    }

   /*  #[tokio::test]
    async fn test_create_view_is_ok() {
        let repo = setup_test_db().await;
        let table_t: CreateTableInfo = CreateTableInfo{table_name:"testablae".to_string(),columns:vec![Column{name:"id".to_string(),data_type:Some("INT".to_string()),is_nullable:Some(true),default_value:Some("".to_string()),is_primary_key:Some(false),maximum_length:Some(0),table_name:"test_t".to_string(),db_name:"test_db".to_string(),type_:"column".to_string()}],db_name:"test_db".to_string(), schema_name: todo!() }};

        let view_info = CreateViewInfo {view_name:"test_view".to_string(),stmt:"SELECT * FROM testablae;".to_string(), columns:vec!["id".to_string()], table_name: "testablae".to_string()};
        let _ = repo.create_table(&table_t).await;  

        let result = repo.create_view(&view_info,"test_db").await;
        println!("result: {:?}",result);
        assert!(result.is_ok());

        let drop_query_t = "DROP TABLE IF EXISTS testablaa;";
        repo.rb_map.get("test_db").unwrap().exec(drop_query_t, vec![]).await.unwrap();
    }*/

    #[tokio::test]
    async fn test_create_view_err_table_not_exist(){
        let repo = setup_test_db().await;
        let view_info = CreateViewInfo {view_name:"test_view".to_string(),stmt:"SELECT * FROM testablae;".to_string(), columns:vec!["id".to_string()], table_name: "testablae".to_string(), schema_name: todo!()};
        let result = repo.create_view(&view_info,"test_db").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_create_role_is_ok(){
        let repo = setup_test_db().await;

        let role_info:Role = Role { name: "test_rolee".to_string(), is_super: Some(false), is_insherit: Some(false), is_create_role: Some(false), is_create_db: Some(true), can_login: Some(true), is_replication: Some(false), connection_limit: Some(100), valid_until: Some("".to_string()), password: Some("valamike".to_string()), db_name: "test_db".to_string(), type_: "role".to_string(),schema_name:todo!() };
        let result = repo.create_role(role_info, "test_db").await.await;

        println!("result: {:?}",result);
        assert!(result.is_ok());

        let drop_query_t = "DROP ROLE IF EXISTS test_role;";
        repo.rb_map.get("test_db").unwrap().exec(drop_query_t, vec![]).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_role_err_database_not_exist(){
        let repo = setup_test_db().await;

        let role_info:Role = Role { name: "test_rolee".to_string(), is_super: Some(false), is_insherit: Some(false), is_create_role: Some(false), is_create_db: Some(true), can_login: Some(true), is_replication: Some(false), connection_limit: Some(100), valid_until: Some("".to_string()), password: Some("valamike".to_string()), db_name: "test_db".to_string(), type_: "role".to_string(),schema_name: todo!() };
        let result = repo.create_role(role_info, "akela_db").await.await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_view_is_ok(){
        let repo = setup_test_db().await;

        //let table_t: CreateTableInfo = CreateTableInfo{table_name:"testablae".to_string(),columns: vec![Column{ name: "id".to_string(), data_type: Some("INT".to_string()), is_nullable: Some(true), default_value: Some("".to_string()), is_primary_key: Some(false), maximum_length: Some(0), table_name: "test_t".to_string(), db_name: "test_db".to_string(), type_: "column".to_string() }],db_name:"test_db".to_string()};
        let view_info = CreateViewInfo {view_name:"test_view".to_string(),stmt:"SELECT * FROM testablae;".to_string(), columns:vec!["id".to_string()], table_name: "testablae".to_string(), schema_name: todo!()};
        let old_view_info = View {name:"test_view".to_string(),definition:"SELECT * FROM testablae;".to_string(),type_:"view".to_string(), schema_name: todo!(), db_name: "test_db".to_string()};
        let new_view_info = View {name:"test_view22".to_string(),definition:"SELECT id FROM testablae;".to_string(),type_:"view".to_string(), schema_name: todo!(), db_name: "test_db".to_string()};
        let _ = repo.create_view(&view_info,"test_db").await;

        let result = repo.edit_view(old_view_info, new_view_info, "test_db").await.await;

        println!("result: {:?}",result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_edit_view_err_old_view_not_exists(){
        let repo = setup_test_db().await;

        let old_view_info = View {name:"test_view".to_string(),definition:"SELECT * FROM testablae;".to_string(),type_:"view".to_string(), schema_name: todo!(), db_name: "test_db".to_string()};
        let new_view_info = View {name:"test_view22".to_string(),definition:"SELECT id FROM testablae;".to_string(),type_:"view".to_string(), schema_name: todo!(), db_name: "test_db".to_string()};

        let result = repo.edit_view(old_view_info, new_view_info, "test_db").await.await;
    
        assert!(result.is_err());
        
    }

    #[tokio::test]
    async fn test_edit_index_is_ok(){
        let repo =  setup_test_db().await;

        let create_index_info = CreateIndexInfo {
            index_name: "test_idxq".to_string(),
            table_name: "test_tabletete".to_string(),
            columns: vec!["id".to_string()],
            schema_name: todo!()
        };
        let old_index_info = Index {
            table_name: Some("test_tabletete".to_string()),
            name: "test_idxq".to_string(),
            definition: Some("".to_string()),
            column_name: Some(vec!["id".to_string()]),
            non_unique: Some(false),
            db_name: "test_db".to_string(),
            type_: "index".to_string(),
            schema_name: todo!()
        };
        let new_index_info = Index {
            table_name: Some("test_tabletete".to_string()),
            name: "test_idxqcq".to_string(),
            definition: Some("".to_string()),
            column_name: Some(vec!["id".to_string()]),
            non_unique: Some(false),
            db_name: "test_db".to_string(),
            type_: "index".to_string(),
            schema_name: todo!()
        };
        let _ = repo.create_index(&create_index_info, "test_db");
        let result = repo.edit_index("test_tabletete", new_index_info, old_index_info, "test_db").await.await;
    
        println!("result: {:?}", result);

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_edit_index_err_old_index_not_exists(){
        let repo =  setup_test_db().await;

        let create_index_info = CreateIndexInfo {
            index_name: "test_idxq".to_string(),
            table_name: "test_tabletete".to_string(),
            columns: vec!["id".to_string()],
            schema_name: todo!()
        };
        let old_index_info = Index {
            table_name: Some("test_tabletete".to_string()),
            name: "test_idxq".to_string(),
            definition: Some("".to_string()),
            column_name: Some(vec!["id".to_string()]),
            non_unique: Some(false),
            db_name: "test_db".to_string(),
            type_: "index".to_string(),
            schema_name: todo!()
        };

        let new_index_info = Index {
            table_name: Some("test_tabletete".to_string()),
            name: "test_idxqcq".to_string(),
            definition: Some("".to_string()),
            column_name: Some(vec!["id".to_string()]),
            non_unique: Some(false),
            db_name: "test_db".to_string(),
            type_: "index".to_string(),
            schema_name: todo!()
        };
        let _ = repo.create_index(&create_index_info, "test_db");
        let result = repo.edit_index("test_tabletete", new_index_info, old_index_info, "test_db").await.await;
    
        assert!(result.is_err());
    }


    #[tokio::test]
    async fn test_edit_sequence_is_ok(){
        let repo = setup_test_db().await;
        let table_t: CreateTableInfo = CreateTableInfo{table_name:"faszaa".to_string(),columns:vec![Column{name:"namajo".to_string(),data_type:Some("INT".to_string()),is_nullable:Some(true),default_value:Some("".to_string()),is_primary_key:Some(false),maximum_length:Some(0),table_name:"test_t".to_string(),db_name:"test_db".to_string(),type_:"column".to_string(), schema_name: todo!() }],db_name:"test_db".to_string(), schema_name: todo!() };
        let seq_info = CreateSequenceInfo {
            sequence_name: "test_sequa".to_string(),
            increment: "1".to_string(),
            minimum_val: "1".to_string(),
            maximum_val: "100".to_string(),
            start_val: "1".to_string(),
            cycle: true,
            schema_name: todo!()
        };
        let _ = repo.create_table(&table_t).await;  
        let _ = repo.create_sequence("faszaa", "test_db","test_db", &seq_info).await;


        let old_seq_info = Sequence {
            name: "test_sequa".to_string(),
            increment: Some("1".to_string()),
            minimum_val: Some("1".to_string()),
            maximum_val: Some("100".to_string()),
            start_val: Some("1".to_string()),
            cycle: Some("CYCLE".to_string()),
            db_name: "test_db".to_string(),
            type_: Some("sequence".to_string()),
            schema_name: todo!()
        };            

        let new_seq = Sequence {
            name: "test_seqaqua".to_string(),
            increment: Some("1".to_string()),
            minimum_val: Some("1".to_string()),
            maximum_val: Some("25".to_string()),
            start_val: Some("1".to_string()),
            cycle: Some("CYCLE".to_string()),
            db_name: "test_db".to_string(),
            type_: Some("sequence".to_string()),
            schema_name: todo!()
        };  

        let result = repo.edit_sequence(old_seq_info, new_seq, "test_db").await.await;
        println!("result: {:?}", result);
    }

    #[tokio::test]
    async fn test_edit_sequence_err_old_seq_not_exists(){
        let repo = setup_test_db().await;

        let old_seq_info = Sequence {
            name: "test_sequa".to_string(),
            increment: Some("1".to_string()),
            minimum_val: Some("1".to_string()),
            maximum_val: Some("100".to_string()),
            start_val: Some("1".to_string()),
            cycle: Some("CYCLE".to_string()),
            db_name: "test_db".to_string(),
            type_: Some("sequence".to_string()),
            schema_name: todo!()
        };            
        let new_seq = Sequence {
            name: "test_seqaqua".to_string(),
            increment: Some("1".to_string()),
            minimum_val: Some("1".to_string()),
            maximum_val: Some("25".to_string()),
            start_val: Some("1".to_string()),
            cycle: Some("CYCLE".to_string()),
            db_name: "test_db".to_string(),
            type_: Some("sequence".to_string()),
            schema_name: todo!()
        };
        let result = repo.edit_sequence(old_seq_info, new_seq, "test_db").await.await;
        assert!(result.is_err());
    }
}