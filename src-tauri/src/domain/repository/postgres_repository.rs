
use crate::domain::metadata::column::Column;
use crate::domain::metadata::constraint::Constraint;
use crate::domain::metadata::database::Schema;
use crate::domain::metadata::function;
use crate::domain::metadata::index::Index;
use crate::domain::metadata::rls_policy::RlsPolicy;
use crate::domain::metadata::role::{Role};
use crate::domain::metadata::rule::Rule;
use crate::domain::metadata::trigger::{Trigger, TriggerFunction};
use crate::domain::metadata::utype::TypeField;
use crate::domain::metadata::{
    function::Function, materalized_view::MateralizedView, procedure::Procedure, table::Table,
    trigger, view::View,
};

use crate::domain::metadata::aggregate::Aggregate;
use crate::domain::metadata::catalog::Catalog;
use crate::domain::metadata::sequence::Sequence;
use crate::domain::repository::database_repository::DatabaseRepository;
use dashmap::mapref::one::Ref;

use futures::stream::Collect;
use rbatis::dark_std::errors::new;
use rbatis::executor::RBatisRef;
use rbatis::rbatis_codegen::ops::AsProxy;
use rbdc::db::ConnectOptions;

use rbdc_pg::*;

use rbs::value::map::ValueMap;
use std::collections::HashMap;

use std::fs;
use std::future::Future;

use rbs::Value;

use std::sync::Arc;

//  use fast_log::{init, print};

use crate::domain::metadata::database_metadata::DatabaseMetadata;
use crate::DatabaseConnection;
use dashmap::DashMap;

use tokio::sync::Semaphore;

use mockall::predicate::*;
use mockall::*;

use crate::domain::create_info::create_table_info::{
    CreateFunctionInfo, CreateIndexInfo, CreateSequenceInfo, CreateTableInfo, CreateViewInfo,
};
#[derive(Debug, Clone)]
pub struct PostgresRepository {
    pub rb_map: DashMap<String, Arc<rbatis::RBatis>>,
    base_url: String,
    pub databases: DashMap<String, DatabaseMetadata>,
    connection_info: DatabaseConnection,
    semaphore: Arc<Semaphore>,
}

impl PostgresRepository {
    pub fn new(connection_info: DatabaseConnection) -> Self {
        let rb_map = DashMap::new();
        let databases = DashMap::new();
        let base_url = String::from(format!(
            "{}://{}:{}@{}:{}/postgres",
            connection_info.driver_type,
            connection_info.username,
            connection_info.password,
            connection_info.server,
            connection_info.port
        ));

        return PostgresRepository {
            rb_map,
            base_url,
            databases,
            connection_info,
            semaphore: Arc::new(Semaphore::new(10)),
        };
    }

    ///Add the database to the pool if not exists
    ///It's create a new rbatis, initialize it add add to the pool
    async fn connect(&self, db_name: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !self.rb_map.contains_key(db_name) {
            log::info!("new pool adding... database: {:?}", db_name);
            let rb = Arc::new(rbatis::RBatis::new());
            match rb.init(PgDriver {}, url) {
                Ok(_) => log::info!("Connection to {} successful", db_name),
                Err(e) => {
                    log::error!("Failed to initialize rbatis for {}: {:?}", db_name, e);
                    return Err(Box::new(e)); // Return the error early
                }
            }

            rb.get_pool().unwrap().set_max_open_conns(100);

            self.rb_map.insert(db_name.to_string(), rb);
            let db_metadata = DatabaseMetadata {
                name: db_name.to_string(),
                schemas: Some(HashMap::new()),
                foreign_data_wrappers: Some(HashMap::new()),
                catalogs: Some(HashMap::new()),
                type_: "database".to_string(),
            };

            self.databases.insert(db_name.to_string(), db_metadata);
            log::info!("Database node created for {}", db_name);
        }
        Ok(())
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
                                    .get_constraints(db_namae, table_namae, schema_namae)
                                    .await
                                    .unwrap();
                                self.init_constraints(&constr, db_namae, schema_namae, table_namae)
                                    .await;
                                let table_triggers = self.get_triggers_associated_with_table(db_namae, table_namae)
                                                                    .await
                                                                    .unwrap();
                                self.init_trigger_functions_for_table(&table_triggers, db_namae, table_namae,schema_namae).await;
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
                        let fdw = self.get_foreign_data_wrappers(db_namae).await.unwrap();
                        self.init_foreign_data_wrappers(&fdw, db_namae).await;
                        let typez = self.get_types(db_namae).await.unwrap();
                        self.init_types(&typez, db_namae, schema_namae).await;
                        let functions = self.get_functions(db_namae, schema_namae).await.unwrap();
                        self.init_functions(&functions, db_namae,schema_namae).await;
                        let roles = self.get_roles_and_users(db_namae).await.unwrap();
                        self.init_roles(&roles, db_namae, schema_namae).await;
                    }
                }
            }
        }
    }

    pub async fn init_db2(&self) {
        let databases = self.get_databases().await.unwrap();
        let schemas: Vec<ValueMap> = Vec::new();
        let mut tables: Vec<ValueMap> = Vec::new();
        let mut columns: Vec<ValueMap> = Vec::new();

        for dab in databases {
            for db_name in dab.0 {
                let db_namae = db_name.1.as_str().unwrap();
                let schema_namae = "public";
                let mut node = self.databases.get_mut(db_namae).unwrap();
                let schemamap = node.value_mut().schemas.get_or_insert_with(HashMap::new);
                let schema_node = crate::domain::metadata::database::Schema {
                    name: schema_namae.to_string(),
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
                schemamap.insert(schema_namae.to_string(), schema_node);

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
                        let table_triggers = self.get_triggers_associated_with_table(db_namae, table_namae)
                            .await
                            .unwrap();
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
              //  self.init_trigger_functions(&trigfuncs, db_namae).await;
                let seqs = self.get_sequences(db_namae, schema_namae).await.unwrap();
                self.init_sequences(&seqs, db_namae, schema_namae).await;
                let matv = self.get_materalized_views(db_namae).await.unwrap();
                self.init_materalized_views(&matv, db_namae, schema_namae)
                    .await;
                let fdw = self.get_foreign_data_wrappers(db_namae).await.unwrap();
                self.init_foreign_data_wrappers(&fdw, db_namae).await;
                let typez = self.get_types(db_namae).await.unwrap();
                self.init_types(&typez, db_namae, schema_namae).await;
                let functions = self.get_functions(db_namae, &schema_namae).await.unwrap();
                self.init_functions(&functions, db_namae,schema_namae).await;
            }
        }
    }

    ///Get the database by its name
    pub fn get_database_(
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
    ) -> Result<Option<Ref<'_, String, Arc<rbatis::RBatis>>>, rbdc::error::Error> {
        let cached_rb = self.rb_map.get(db_name);
        if cached_rb.is_some() && !cached_rb.as_ref().unwrap().get_pool().is_err() {
            log::info!("rb cached: {:?}", db_name);
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
            return Err(rbdc::Error::from("Failed to connect to database"));
        }
        match self.rb_map.get(db_name) {
            Some(rb) => {
                log::info!("Connection successfull, rb cached: {:?}", db_name);
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
                        log::info!(
                            "Column is nullable? {:?}",
                            col.0
                                .get(&Value::String("is_nullable".to_string()))
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                        );
                        let nullable = col
                            .0
                            .get(&Value::String("is_nullable".to_string()))
                            .and_then(|v| v.as_str())
                            .unwrap_or_default();
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
                        let is_nullable = if nullable == "YES" { true } else { false };
                        let column_default = col
                            .0
                            .get(&Value::String("column_default".to_string()))
                            .and_then(|v| v.as_str())
                            .unwrap_or_default();
                        let is_primary_ret = col
                            .0
                            .get(&Value::String("is_primary_key".to_string()))
                            .and_then(|v| v.as_str())
                            .unwrap_or_default();
                        let maximum_length = col
                            .0
                            .get(&Value::String("character_maximum_length".to_string()))
                            .and_then(|v| v.as_i64())
                            .unwrap_or_default();
                        let is_primary = if is_primary_ret == "YES" { true } else { false };

                        let _col_node = crate::domain::metadata::column::Column {
                            name: String::from(col_name), //col_name.1.to_string(),
                            data_type: Some(String::from(data_type)),
                            is_nullable: Some(is_nullable),
                            default_value: Some(String::from(column_default)),
                            is_primary_key: Some(is_primary),
                            maximum_length: Some(maximum_length),
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
    }

    async fn init_views(&self, result2: &Vec<ValueMap>, db_name: &str, schema_name: &str) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            if let Some(schemas) = node.schemas.as_mut().unwrap().get_mut(schema_name) {
                let view_map = schemas.views.get_or_insert_with(HashMap::new); //node.value_mut().views.get_or_insert_with(HashMap::new);
                for view in result2 {
                    let view_name = view
                        .0
                        .get(&Value::String("table_name".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let view_definition = view
                        .0
                        .get(&Value::String("view_definition".to_string()))
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
                        .get(&Value::String("arguments".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let mut stored_procedure_node = Procedure {
                        name: stored_procedure_name.to_string(),
                        definition: stored_procedure_definition.to_string(),
                        parameters: Some(Vec::new()),
                        source_db: crate::domain::datb::database_type::DatabaseType::Postgres,
                        type_: "procedure".to_string(),
                        schema_name:Some(schema_name.to_string()),
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

    async fn init_functions(&self, result2: &Vec<ValueMap>, db_name: &str,schema_name: &str) {
        //let mut db_struct = self.databases.lock().unwrap();
        if let Some(mut node) = self.databases.get_mut(db_name) {
            let function_map = node
                .schemas
                .as_mut()
                .unwrap()
                .get_mut(schema_name)
                .unwrap()
                .functions
                .get_or_insert_with(HashMap::new);
            for function in result2 {
                log::info!("Current function info: {:?}", function);
                let function_name = function
                    .0
                    .get(&Value::String("function_name".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let function_def = function
                    .0
                    .get(&Value::String("function_body".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let params = function
                    .0
                    .get(&Value::String("arguments".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let return_type = function
                    .0
                    .get(&Value::String("return_type".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let full_function = function
                    .0
                    .get(&Value::String("full_function".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                log::info!("FULL FUNCTION STUFF: {:?}", full_function);
                let mut function_node = Function {
                    name: function_name.to_string(),
                    definition: full_function.to_string(),
                    parameters: Some(Vec::new()),
                    return_type: Some(return_type.to_string()),
                    type_: Some("function".to_string()),
                    schema_name:Some(schema_name.to_string()),
                    db_name: db_name.to_string(),
                    full_function: Some(full_function.to_string()),
                };
                function_node.parameters = Some(
                    params
                        .split(',')
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>(),
                );
                function_map.insert(function_name.to_string(), function_node);
            }
        } else {
            log::info!("Node is not OK");
        }
    }

    async fn init_trigger_functions_for_table(&self, result2: &Vec<ValueMap>, db_name: &str,table_name:&str,schema_name: &str) {

       // if let Some(mut node) = self.databases.get_mut(db_name) {
            if let Some(mut node) = self.databases.get_mut(db_name) {
                log::info!("Node is OK");
                if let Some(schemas) = node.schemas.as_mut().unwrap().get_mut(schema_name) {
                    log::info!("Schema is OK");
                    if let Some(table_map) = schemas.tables.as_mut().unwrap().get_mut(table_name) {
                        log::info!("Table is OK");
                        let trigger_map = table_map.triggers.get_or_insert_with(HashMap::new);

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
                            let trigger_node = trigger::Trigger {
                                name: trigger_name.to_string(),
                                definition : trigger_definition.to_string(),
                                type_: "trigger".to_string(),
                                schema_name:schema_name.to_string(),
                                db_name: db_name.to_string(),
                            };
                            trigger_map.insert(trigger_name.to_string(), trigger_node);
                        }
                    }else{
                        log::info!("Table is not OK");
                    }
                }else{
                    log::info!("Schema is not OK");
                }
            }else{
                log::info!("Node is not OK");
            }
        }


            /*let trigger_map = node
                .value_mut()
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
                .triggers
                .get_or_insert_with(HashMap::new);
            for trigger in result2 {
                let trigger_name = trigger
                    .0
                    .get(&Value::String("Trigger".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let trigger_definition = trigger
                    .0
                    .get(&Value::String("trigger_definition".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let trigger_node = trigger::Trigger {
                    name: trigger_name.to_string(),
                    definition : trigger_definition.to_string(),
                    type_: "trigger".to_string(),
                };
                trigger_map.insert(trigger_name.to_string(), trigger_node);
            }
        } else {
            log::info!("Node is not OK");
        }*/
   // }

    async fn init_trigger_functions(&self, result2: &Vec<ValueMap>, db_name: &str, schema_name: &str) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            let trigger_map = node
                .value_mut()
                .schemas
                .as_mut()
                .unwrap()
                .get_mut(schema_name)
                .unwrap()
                .triggers
                .get_or_insert_with(HashMap::new);
            for trigger in result2 {
                let trigger_name = trigger
                    .0
                    .get(&Value::String("trigger_name".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let function_name = trigger
                    .0
                    .get(&Value::String("function_name".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let trigger_definition = trigger
                    .0
                    .get(&Value::String("function_definition".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let trigger_node = trigger::TriggerFunction {
                    name: function_name.to_string(),
                    definition : trigger_definition.to_string(),
                    type_: "triggerfunction".to_string(),
                    schema_name:schema_name.to_string(),
                    db_name: db_name.to_string(),
                };
                trigger_map.insert(trigger_name.to_string(), trigger_node);
            }
        } else {
            log::info!("Node is not OK");
        }
    }

    async fn init_aggregates(&self, result2: &Vec<ValueMap>, db_name: &str, schema_name: &str) {
        if let Some(mut node) = self.databases.get_mut(db_name) {
            let aggregate_map = node
                .value_mut()
                .schemas
                .as_mut()
                .unwrap()
                .get_mut(schema_name)
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

    async fn init_foreign_data_wrappers(&self, result2: &Vec<ValueMap>, db_name: &str) {
        //let mut db_struct = self.databases.lock().unwrap();
        if let Some(mut node) = self.databases.get_mut(db_name) {
            let foreign_data_wrapper_map = node
                .value_mut()
                .foreign_data_wrappers
                .get_or_insert_with(HashMap::new);
            for foreign_data_wrapper in result2 {
                let foreign_data_wrapper_name = foreign_data_wrapper
                    .0
                    .get(&Value::String("fdwname".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();

                let foreign_data_wrapper_node =
                    crate::domain::metadata::foreign_data_wrapper::ForeignDataWrapper {
                        name: foreign_data_wrapper_name.to_string(),
                    };

                foreign_data_wrapper_map.insert(
                    foreign_data_wrapper_name.to_string(),
                    foreign_data_wrapper_node,
                );
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
                    .get(&Value::String("schema_name".to_string()))
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

    async fn init_indexes(
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
                .as_mut()
                .unwrap()
                .get_mut(schema_name)
            {
                if let Some(table_map) = schemas.tables.as_mut().unwrap().get_mut(table_name) {
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
                                .as_mut()
                                .unwrap()
                                .push(column_name.to_string());
                        } else {
                            let table_name = index
                                .0
                                .get(&Value::String("table_name".to_string()))
                                .and_then(|v| v.as_str())
                                .unwrap_or_default();
                            let non_unique = index
                                .0
                                .get(&Value::String("is_unique".to_string()))
                                .and_then(|v| v.as_bool())
                                .unwrap_or_default();

                            let mut index_node = crate::domain::metadata::index::Index {
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
                                .as_mut()
                                .unwrap()
                                .push(column_name.to_string());
                            index_map.insert(index_name.to_string(), index_node);
                        }
                    }
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
                .as_mut()
                .unwrap()
                .get_mut(schema_name)
            {
                if let Some(table_map) = schemas.tables.as_mut().unwrap().get_mut(table_name) {
                    //node.value_mut().tables.clone().unwrap().get_mut(table_name){
                    let constraint_map = table_map.constraints.get_or_insert_with(HashMap::new);
                    //Constraints::{"actor_pkey": Constraint { name: "actor_pkey", c_type: "PRIMARY KEY", table_name: "actor", column_name: "actor_id", db_name: "test_db" }}
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
                            fk_column: "".to_string(),
                            fk_table: "".to_string(),
                            db_name: db_name.to_string(),
                            schema_name:Some(schema_name.to_string()),
                            type_:"constraint".to_string(),
                        };
                        constraint_map.insert(constraint_name.to_string(), constraint_node);
                    }
                    log::info!("Constraints::{:?}",constraint_map);
                }
            }
        } else {
            log::info!("Node is not OK");
        }
    }

    async fn init_roles(&self, result2: &Vec<ValueMap>, db_name: &str,schema_name:&str){
        if let Some(mut node) = self.databases.get_mut(db_name){
            if let Some(schemas) = node.schemas.as_mut().unwrap().get_mut(schema_name){
                let role_map = schemas.roles.get_or_insert_with(HashMap::new);
                for role in result2{
                    let name= role
                    .0
                    .get(&Value::String("role_name".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                    
                    let is_super = role
                    .0
                    .get(&Value::String("is_superuser".to_string()))
                    .and_then(|v| v.as_bool())
                    .unwrap_or_default();
                    let can_inherit = role
                    .0
                    .get(&Value::String("can_inherit".to_string()))
                    .and_then(|v| v.as_bool())
                    .unwrap_or_default();
                    let can_create_role = role
                    .0
                    .get(&Value::String("can_create_roles".to_string()))
                    .and_then(|v| v.as_bool())
                    .unwrap_or_default();
                    let can_create_db = role
                    .0
                    .get(&Value::String("can_create_db".to_string()))
                    .and_then(|v| v.as_bool())
                    .unwrap_or_default();
                    let can_login = role
                    .0
                    .get(&Value::String("can_login".to_string()))
                    .and_then(|v| v.as_bool())
                    .unwrap_or_default();
                    let can_replicate = role
                    .0
                    .get(&Value::String("can_replicate".to_string()))
                    .and_then(|v| v.as_bool())
                    .unwrap_or_default();
                    let connection_limit = role
                    .0
                    .get(&Value::String("connection_limit".to_string()))
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default();
                    let valid_until = role
                        .0
                        .get(&Value::String("valid_until".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();

                    let role_node:Role = Role{
                        name: name.to_string(),
                        is_super: Some(is_super),
                        is_insherit: Some(can_inherit),
                        is_create_role: Some(can_create_role),
                        is_create_db: Some(can_create_db),
                        can_login: Some(can_login),
                        is_replication: Some(can_replicate),
                        connection_limit: Some(connection_limit.i32()),
                        valid_until: Some(valid_until.to_string()),
                        password : None,
                        db_name: db_name.to_string(),
                        type_: "role".to_string(),
                        schema_name: schema_name.to_string(),
                    };
                    role_map.insert(name.to_string(), role_node);
                        
                }
            }
        }
    }
/*

SELECT
            rolname AS role_name,
            rolsuper AS is_superuser,
            rolinherit AS can_inherit,
            rolcreaterole AS can_create_roles,
            rolcreatedb AS can_create_db,
            rolcanlogin AS can_login,
            rolreplication AS can_replicate,
            rolconnlimit AS connection_limit,
            rolvaliduntil AS valid_until
        FROM
            pg_roles;

*/
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
                    let start_val = seq
                        .0
                        .get(&Value::String("start_value".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let min_val = seq
                        .0
                        .get(&Value::String("minimum_value".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let max_val = seq
                        .0
                        .get(&Value::String("maximum_value".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let increment = seq
                        .0
                        .get(&Value::String("increment".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let cycle = seq
                        .0
                        .get(&Value::String("cycle".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let sequence_node = Sequence {
                        name: sequence_name.to_string(),
                        start_val: Some(start_val.to_string()),
                        increment: Some(increment.to_string()),
                        cycle: Some(cycle.to_string()),
                        minimum_val: Some(min_val.to_string()),
                        maximum_val: Some(max_val.to_string()),
                        type_: Some("sequence".to_string()),
                        schema_name:Some(schema_name.to_string()),
                        db_name: db_name.to_string(),
                    };
                    sequence_map.insert(sequence_name.to_string(), sequence_node);
                }
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
                            .get(&Value::String("rule_definition".to_string()))
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

    fn get_primary_key<'a>(
        &'a self,
        table_name: &'a str,
        db_name: &'a str,
    ) -> impl Future<Output = Result<Vec<Value>, rbdc::Error>> + Send + 'a {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();
            let _sql = "SELECT 
                    kcu.column_name
                FROM 
                    information_schema.table_constraints tc
                JOIN 
                    information_schema.key_column_usage kcu
                    ON tc.constraint_name = kcu.constraint_name
                    AND tc.table_name = kcu.table_name
                WHERE 
                    tc.constraint_type = 'PRIMARY KEY'
                    AND kcu.table_name = ?;";
            let result: Vec<Value> = rb
                .query_decode(_sql, vec![Value::String(table_name.to_string())])
                .await?;
            Ok(result)
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

    pub fn create_database<'a>(
        &'a self,
        db_name: &'a str,
        file: &'a str,
    ) -> impl Future<Output = Result<(), rbdc::Error>> + Send + 'a {
        async move { 
            let rb = self.rbatis_connect("postgres").await?.unwrap();
        /* let _sql = if(file.len() <= 0){
                format!("CREATE DATABASE {};",db_name)
            }else{
                format!("{}",file)
            }; */
            let _sql = format!("{}",file);
            let result = rb.query(&_sql, vec![]).await;
            log::info!("create database result is ok: {:?}",result.is_ok());
            if result.is_ok(){
                return Ok(())
            }else{
                return Err(result.unwrap_err())
            }
        }
    }
    pub fn create_table<'a>(
        &'a self,
        db_name: &'a str,
        table_info: &'a CreateTableInfo,
    ) -> impl Future<Output = Result<Table, rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(db_name) {
                let mut col_transformed_datas: Vec<String> = Vec::new();
                let mut pk = String::new();
                let mut is_nullable = String::new();
                let mut col_map: HashMap<String, Column> = HashMap::new();
                for col in &table_info.columns {
                    if col.is_primary_key.unwrap() {
                        pk = "PRIMARY KEY".to_string();
                    } else if col.maximum_length != Some(0) {
                        pk = format!("({:?})", col.maximum_length.unwrap_or(240));
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
                let result: Result<(), rbdc::Error> = rb.query_decode(&_sql, vec![]).await;
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
                    schema_name:Some(table_info.schema_name.clone()),
                    db_name: table_info.table_name.clone(),
                };
                let table_clone = new_table.clone();
                println!("Create Table info: {:?}", table_clone);
                println!("dbname: {}", &db_name);
                self.databases
                    .get_mut(&db_name.to_string())
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut(new_table.schema_name.as_ref().unwrap())
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
            log::info!("Database name: {}", database_name);
            if let Some(rb) = self.rb_map.get("test_db") {
                let mut _sql = String::new();
                if create_seq_info.cycle {
                    _sql = format!("CREATE SEQUENCE {} INCREMENT BY {} MINVALUE {} MAXVALUE {} START WITH {} CYCLE;",create_seq_info.sequence_name,create_seq_info.increment,create_seq_info.minimum_val,create_seq_info.maximum_val,create_seq_info.start_val);
                } else {
                    _sql = format!("CREATE SEQUENCE {} INCREMENT BY {} MINVALUE {} MAXVALUE {} START WITH {} NO CYCLE;",create_seq_info.sequence_name,create_seq_info.increment,create_seq_info.minimum_val,create_seq_info.maximum_val,create_seq_info.start_val);
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
                    schema_name:Some(schema_name.to_string()),
                    db_name: database_name.to_string(),
                };
                 self.databases
                    .get_mut(database_name)
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut(schema_name)
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
                    "CREATE FUNCTION IF NOT EXISTS {}({:?})\n{}",
                    create_function.name, create_function.parameters, create_function.definition
                );
                let result_: Result<(), rbdc::Error> = rb.query_decode(&_sql, vec![]).await.unwrap();

                let funtion_created = create_function.clone();

                self.databases
                    .get_mut(database_name)
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut(create_function.schema_name.as_ref().unwrap())
                    .unwrap()
                    .functions
                    .as_mut()
                    .unwrap()
                    .insert(create_function.name.clone(), create_function.clone());
                Ok(funtion_created)
            } else {
                Err(rbdc::Error::from("database not found!"))
            }
        }
    }

    /* 
    pub async fn create_procedure<'a>(
        &'a self,
        procedure:Procedure
    )->impl Future<Output = Result<Procedure, rbdc::Error>> + Send + 'a{
            async  move{
                if let Some(rb) = self.rb_map.get(&procedure.db_name) {
                    let sql = format!("CREATE PROCEDURE {}({}) {}",procedure.name,procedure.parameters.unwrap().join(","),procedure.definition);
                    let result_: Result<(), rbdc::Error> = rb.query_decode(&sql, vec![]).await.unwrap();
                    let proc = procedure.clone();
                    self.databases
                    .get_mut(&procedure.db_name)
                    .unwrap()
                    .schemas
                    .unwrap()
                    .get_mut(&procedure.schema_name.unwrap())
                    .unwrap()
                    .procedures
                    .insert(procedure.name,proc.clone());
                    Ok(proc)
                }else{
                    Err(rbdc::Error::from("database not found!"))
                }
        }
    }*/

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
                println!("sql: {}", &_sql);
                let result_:Result<(), rbdc::Error>  = rb.query_decode(&_sql, vec![]).await;
                let index: Index = Index{
                    name: index_info.index_name.clone(),
                    definition: todo!(),
                    column_name: Some(index_info.columns.clone()),
                    non_unique: todo!(),
                    table_name: Some(index_info.table_name.clone()),
                    db_name: database_name.to_string(),
                    schema_name: Some("public".to_string()),
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
                Ok(index)
             //.unwrap()
            } else {
                Err(rbdc::Error::from("database not found"))
            }
        }
    }

    pub fn create_view<'a>(
        &'a self,
        view_info: &'a CreateViewInfo,
        database_name: &'a str,
        schema_name: &'a str
    ) -> impl Future<Output = Result<View, rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(database_name) {
                let mut _sql = String::new();
                _sql = format!("CREATE VIEW {} AS {}", view_info.view_name, view_info.stmt);
                let result:Result<(), rbdc::Error> = rb.query_decode(&_sql, vec![]).await;
                let view: View = View {
                    name: view_info.view_name.clone(),
                    definition: view_info.stmt.clone(),
                    type_: "view".to_string(),
                    db_name: database_name.to_string(),
                    schema_name: schema_name.to_string(),
                };

                self.databases
                    .get_mut(database_name)
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut(schema_name)
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
        database_name: &'a str,
        schema_name: &'a str
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
                let result:Result<rbs::Value, rbdc::Error> = rb.query(&sql, vec![]).await;
                if result.is_ok(){
                    log::info!("create role result: {:?}", result);
                    
                    let created_role = Role {
                        name: role_info.name.to_string(),
                        can_login: role_info.can_login,
                        is_super: role_info.is_super,
                        is_create_db: role_info.is_create_db,
                        is_create_role: role_info.is_create_role,
                        connection_limit: role_info.connection_limit,
                        valid_until: role_info.valid_until,
                        password: role_info.password,
                        is_insherit: role_info.is_insherit,
                        is_replication: role_info.is_replication,
                        db_name: role_info.db_name,
                        type_: "role".to_string(),
                        schema_name: schema_name.to_string(),
                    };
                    
                    self.databases
                        .get_mut(database_name)
                        .unwrap()
                        .schemas
                        .as_mut()
                        .unwrap()
                        .get_mut(schema_name)
                        .unwrap()
                        .roles
                        .as_mut()
                        .unwrap()
                        .insert(role_info.name.to_string(), created_role.clone());

                    Ok(created_role)
                }else{
                    log::info!("create role error: {:?}", result);
                    Err(rbdc::Error::from("Failed to create role"))
                }
            }else{
                Err(rbdc::Error::from("database not found"))
            }
        }
       
    }


     pub async fn create_constraint<'a>(
        &'a self,
        constraint: Constraint,
        database_name: &'a str,
        schema_name: &'a str,
        table_name: &'a str,
    ) -> impl Future<Output = Result<Constraint, rbdc::Error>> + Send + 'a {
        async move{
            log::info!("Database name: {}", database_name);
            if let Some(rb) = self.rb_map.get(database_name){

                let create_sql = match constraint.c_type.as_str() {
                    "FOREIGN KEY" => {
                        format!("ALTER TABLE {} ADD CONSTRAINT FOREIGN KEY {} ({});",table_name,constraint.name,constraint.column_name)
                    },
                    "UNIQUE" => {
                        format!("ALTER TABLE {} ADD CONSTRAINT UNIQUE {} ({});",table_name,constraint.name,constraint.column_name)
                    },
                    "CHECK" => {
                        format!("ALTER TABLE {} ADD CONSTRAINT CHECK {} ({});",table_name,constraint.name,constraint.column_name)
                    },
                    "PRIMARY KEY" => {
                        format!("ALTER TABLE {} ADD CONSTRAINT PRIMARY KEY {} ({});",table_name,constraint.name,constraint.column_name)
                    },
                    _ => {
                        format!("ALTER TABLE {} ADD CONSTRAINT -IDK- {} ({});",table_name,constraint.name,constraint.column_name)
                    }
                };
                let create_result:Result<(), rbdc::Error> = rb.query_decode(&create_sql, vec![]).await;
                    let created_constraint = Constraint {
                        name: constraint.name.to_string(),
                        c_type: constraint.c_type.to_string(),
                        table_name: table_name.to_string(),
                        column_name: constraint.column_name.to_string(),
                        db_name: database_name.to_string(),
                        schema_name: Some(schema_name.to_string()),
                        type_: "constraint".to_string(),
                        fk_column: "".to_string(),
                        fk_table: "".to_string()
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
                        .unwrap()
                        .constraints
                        .as_mut()
                        .unwrap()
                        .insert(constraint.name.to_string(), created_constraint.clone());
                    Ok(created_constraint)
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
                    format!("CREATE SCHEMA IF NOT EXISTS {} AUTHORIZATION {}", schema_create_name, user)
                } else {
                    format!("CREATE SCHEMA IF NOT EXISTS {}", schema_create_name)
                };
                
                let result:Result<(), rbdc::Error> = rb.query_decode(&sql, vec![]).await;
                let schema: Schema = Schema {
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
                    materalized_views: Some(HashMap::new()),
                    catalogs: Some(HashMap::new()),
                };
                self.databases
                    .get_mut(database_name)
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .insert(schema_create_name.to_string(), schema.clone());
                if result.is_ok(){
                    log::info!("create schema result: {:?}", result);
                    Ok(schema)
                }else{
                    log::info!("create schema error: {:?}", result);
                    Err(rbdc::Error::from("Failed to create schema"))
                }
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
                    name: todo!(),
                    definition: todo!(),
                    type_: todo!(),
                    db_name: todo!(),
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
                Ok(trigger)
            }else{
                Err(rbdc::Error::from("database not found"))
            }

        }
    }

    //triggerfunction
   /*  pub async fn create_triggerfunction<'a>(
        &'a self,
        database_name: &'a str,
        schema_name: &'a str,
        create_function: TriggerFunction,
    ) -> impl Future<Output = Result<TriggerFunction, rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(database_name) {
                let mut _sql = format!(
                    "CREATE FUNCTION IF NOT EXISTS RETURNS TRIGGER AS ${}$ \n{} \nLANGUAGE plpgsql",
                );
                let result_: Result<(), rbdc::Error> = rb.query_decode(&_sql, vec![]).await.unwrap();

                let funtion_created = create_function.clone();

                self.databases
                    .get_mut(database_name)
                    .unwrap()
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut(schema_name)
                    .unwrap()
                    .triggers
                    .as_mut()
                    .unwrap()
                    .insert(create_function.name.clone(), create_function.clone());
                Ok(funtion_created)
            } else {
                Err(rbdc::Error::from("database not found!"))
            }
        }
    }
*/
   /*  pub fn create_function<'a>(
        &'a self,
        func_info: &'a CreateFunctionInfo,
        database_name: &'a str,
    ) -> impl Future<Output = Result<Function, rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(database_name) {
                let _sql = format!(
                    "CREATE OR REPLACE FUNCTION {}({}) {}",
                    func_info.function_name,
                    func_info.params.join(","),
                    func_info.stmt
                );
                let result: Value = rb.query_decode(&_sql, vec![]).await.unwrap();
                let function: Function = Function {
                    name: todo!(),
                    definition: todo!(),
                    type_: todo!(),
                    return_type: todo!(),
                    parameters: todo!(),
                    db_name: todo!(),
                };
                Ok(function)
            } else {
                Err(rbdc::Error::from("database not found"))
            }
        }
    }*/



    pub fn add_column<'a>(
        &'a self,
        table_name: &'a str,
        col: Column,
        database_name: &'a str,
        schema_name: &'a str
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
                    .get_mut(schema_name)
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

    pub async fn alter_table_column<'a>(
        &'a self,
        table_name: &'a str,
        new_col: Column,
        old_col: Column,
        database_name: &'a str,
        schema_name: &'a str
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
                            "ALTER TABLE {} RENAME COLUMN {} TO {};",
                            table_name, old_col.name, new_col.name
                        )
                    } else {
                        "".to_string()
                    };
                    let new_type = new_col.data_type.clone().unwrap();
                    let col_type = if new_col.data_type.unwrap() != old_col.data_type.unwrap() {
                        format!(
                            "ALTER TABLE {} ALTER COLUMN {} TYPE {};",
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
                            "ALTER TABLE {} ALTER COLUMN {} TYPE VARCHAR({});",
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
                                "ALTER TABLE {} ALTER COLUMN {} SET DEFAULT '{}';",
                                table_name, old_col.name, new_default
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
                    let result:Result<(),rbdc::Error> = rb.query_decode(&alter2_sql, vec![]).await.unwrap();
                    log::info!("result: {:?}", result);
                   // let new_col_clone = new_col.clone();
                    *self
                        .databases
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
                        .unwrap()
                        .columns
                        .as_mut()
                        .unwrap()
                        .get_mut(old_col.name.as_str())
                        .unwrap() = new_col_clone.clone();
                    //Ok(new_table)
                    return Ok(new_col_clone);
                }
            }
            return Err(rbdc::Error::from("Failed to connect to database"));
        }
    }

   /*  pub async fn edit_table(
        &'a self,
        new_table: Table,
        old_table: Table,
        database_name: &'a str,
        schema_name: &'a str
    )-> impl Future<Output = Result<(), rbdc::Error>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(database_name) {
                if new_table.name != old_table.name {
                    let sql = format!("ALTER TABLE {} RENAME TO {};", old_table.name, new_table.name);
                    let result: Value = rb.query_decode(&sql, vec![]).await.unwrap();
                    log::info!("result: {}", result);
                }
                if new_table.columns != old_table.columns {
                    for col in &new_table.columns {
                        if !old_table.columns.unwrap().contains(col){
                            self.alter_table_column(table_name, new_col, , database_name, schema_name)
                        }
                }

            }
            return Ok(());
        }
      */          

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
                let drop_sql = format!("ALTER TABLE {} DROP CONSTRAINT IF EXISTS {} CASCADE;",old_constraint.table_name,old_constraint.name);
                let create_sql:String = match new_constraint.c_type.as_str() {
                    "FOREIGN KEY" => {
                        format!("ALTER TABLE {} ADD CONSTRAINT {} {} ({});",old_constraint.table_name,new_constraint.name,old_constraint.c_type,old_constraint.column_name)
                    },
                    _ => {
                        format!("ALTER TABLE {} ADD CONSTRAINT {} {} ({});",old_constraint.table_name,new_constraint.name,old_constraint.c_type,old_constraint.column_name)
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
                
                /*if !create_result.is_err(){
                    //behelyezés
                    Ok(new_constraint)
                }else{
                    Err(rbdc::Error::from("Failed to connect to database"))
                }*/
            }else{
                Err(rbdc::Error::from("Failed to connect to database"))
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
                let drop_sql = format!("DROP INDEX IF EXISTS {} CASCADE;",old_index.name);
                let is_unique = if new_index.non_unique.unwrap(){"UNIQUE"}else{""};
                let create_sql = format!("CREATE {} INDEX {} ON {:?} ({:?})",is_unique,new_index.name,new_index.table_name.unwrap(),new_index.column_name.unwrap().join(""));
                let drop_result:Result<(), rbdc::Error> = rb.query_decode(&drop_sql, vec![/*Value::String(old_index.name)*/]).await;
                //if drop_result.is_ok(){
                let create_result:Result<(), rbdc::Error> = rb.query_decode(&create_sql, vec![/*Value::String(new_index.name),Value::String(new_index.table_name.unwrap()),Value::String("valami".to_string())*/]). await;
               // if !create_result.is_err() {
                    let mut database = self.databases.get_mut(database_name).unwrap();
                    let indexes = database
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
                        .indexes
                        .as_mut()
                        .unwrap();
                    let new_index_clone_cl = new_index_clone.clone();
                    indexes.entry(old_index.name.clone()).or_insert(new_index_clone);
                    //behelyezés
                    Ok(new_index_clone_cl)
                }else{
                    Err(rbdc::Error::from("Failed to connect to database"))
                }
                //}
                //else{
                  //  Ok(())
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
                        "ALTER SEQUENCE {} RESTART WITH {};",
                        old_sequence.name,
                        new_sequence.start_val.unwrap()
                    )
                } else {
                    "".to_string()
                };
                let new_cycle = if new_sequence.cycle != old_sequence.cycle {
                    format!("ALTER SEQUENCE {} NOCYCLE;", old_sequence.name)
                } else {
                    "".to_string()
                };

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
                    {}
                    COMMIT;
                    ",
                    new_name, new_increment, new_max, new_min, new_start, new_cycle
                );
                log::info!("SQL SQL: {}", alter2_sql);
                let result: Result<(), rbdc::Error> = rb.query_decode(&alter2_sql, vec![]).await;
                let mut database = self.databases.get_mut(database_name).unwrap();
                let new_seq_clone_cl = new_seq_clone.clone();
                let seqs = database
                    .schemas
                    .as_mut()
                    .unwrap()
                    .get_mut(&old_sequence.schema_name.unwrap())
                    .unwrap()
                    .sequences
                    .as_mut()
                    .unwrap();
                seqs.entry(old_sequence.name).or_insert(new_seq_clone);
                Ok(new_seq_clone_cl)
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
                let create_sql = format!("CREATE OR REPLACE VIEW {} AS {};",old_view.name,new_view.definition);
                let create_or_replace_result:Result<(),rbdc::Error> = rb.query_decode(&create_sql, vec![]).await;
                if !create_or_replace_result.is_err(){

                    //behelyezés
                    Ok(new_view)
                }else{
                    Err(create_or_replace_result.unwrap_err())
                }
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
                    let drop_sql = format!("DROP FUNCTION IF EXISTS {} CASCADE;", old_function.name);
                    let result_drop: Value = rb.query_decode(&drop_sql, vec![]).await.unwrap();
                    let create_sql = format!(
                        "CREATE OR REPLACE FUNCTION {}({})RETURNS {} AS $$\n{} $$ LANGUAGE plpgsql;",
                        new_function.name, new_function.parameters.unwrap().join(", "),old_function.return_type.unwrap() ,new_function.definition
                    );
                    let result_create: Result<(), rbdc::Error> = rb.query_decode(&create_sql, vec![]).await;
                    *self
                        .databases
                        .get_mut(database_name)
                        .unwrap()
                        .schemas
                        .as_mut()
                        .unwrap()
                        .get_mut(new_function.schema_name.unwrap().as_str())
                        .unwrap()
                        .functions
                        .as_mut()
                        .unwrap()
                        .get_mut(old_function.name.as_str())
                        .unwrap() = new_func_clone.clone();
                    log::info!("result info create function: {:#?}", result_create);
                    Ok(new_func_clone)
                   /*  if !result_create.is_err(){
                        Ok(new_func_clone)
                    }else{
                        Err(result_create.unwrap_err())
                    }*/
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

    pub async fn delete_table<'a>(
        &'a self,
        table_name: &'a str,
        db_name: &'a str,
    ) -> impl Future<Output = Result<(), ()>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(db_name) {
                let sql = format!("DROP TABLE {};", table_name);
                let result: Value = rb.query_decode(&sql, vec![]).await.unwrap();
                Ok(())
            } else {
                Err(())
            }
        }
    }

    pub async fn delete_sequence<'a>(
        &'a self,
        sequence_name: &'a str,
        db_name: &'a str,
    ) -> impl Future<Output = Result<(), ()>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(db_name) {
                let sql = format!("DROP SEQUENCE {};", sequence_name);
                let result: Value = rb.query_decode(&sql, vec![]).await.unwrap();
                Ok(())
            } else {
                Err(())
            }
        }
    }

    pub async fn delete_function<'a>(
        &'a self,
        function_name: &'a str,
        db_name: &'a str,
    ) -> impl Future<Output = Result<(), ()>> + Send + 'a {
        async move {
            if let Some(rb) = self.rb_map.get(db_name) {
                let sql = format!("DROP FUNCTION {};", function_name);
                let result: Value = rb.query_decode(&sql, vec![]).await.unwrap();
                Ok(())
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

    pub async fn get_table<'a>(
        &'a self,
        table_name: &'a str,
        database_name: &'a str,
    ) -> impl Future<Output = Result<Table, ()>> + Send + 'a {
        async move {
            if let Some(db) = self.databases.get(database_name) {
                if let Some(schema) = db.schemas.as_ref().unwrap().get("public") {
                    if let Some(table) = schema.tables.as_ref().unwrap().get(table_name) {
                        Ok(table.clone())
                    } else {
                        Err(())
                    }
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        }
    }
}

impl DatabaseRepository for PostgresRepository {
    ///Get the attached databases for the sqlite file
    fn get_databases(&self) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            log::info!("PgRepository: Get databases");
            let rb = self.rbatis_connect("postgres").await?.unwrap();
            let _sql = "SELECT datname 
                FROM pg_database 
                WHERE datistemplate=false;";
            let result2: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

            Ok(result2)
        }
    }

    ///Get all tables in the database
    fn get_tables(
        &self,
        db_name: &str,
        schema_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            log::info!("Get tables...");
            let rb = self.rbatis_connect(db_name).await?.unwrap();
            let _sql = "SELECT table_name
                FROM information_schema.tables
                WHERE table_schema = ?;";
            let result_2: Vec<ValueMap> = rb
                .query_decode(_sql, vec![Value::String(schema_name.to_string())])
                .await
                .unwrap();
            Ok(result_2)
        }
    }

    ///Get all columns in the table
    fn get_columns(
        &self,
        db_name: &str,
        table_name: &str,
        schema_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT 
                                    c.column_name,
                                    c.data_type,
                                    c.is_nullable,
                                    c.column_default,
                                    c.character_maximum_length,
                                    CASE WHEN kcu.column_name IS NOT NULL THEN 'YES' ELSE 'NO' END AS is_primary_key
                                FROM 
                                    information_schema.columns c
                                LEFT JOIN 
                                    information_schema.table_constraints tc 
                                    ON c.table_name = tc.table_name 
                                    AND tc.constraint_type = 'PRIMARY KEY'
                                LEFT JOIN 
                                    information_schema.key_column_usage kcu 
                                    ON tc.constraint_name = kcu.constraint_name 
                                    AND c.column_name = kcu.column_name
                                WHERE 
                                    c.table_name = ?;";
            let result_2: Vec<ValueMap> = rb
                .query_decode(_sql, vec![Value::String(table_name.to_string())])
                .await
                .unwrap();

            Ok(result_2)
        }
    }

    fn get_views(
        &self,
        db_name: &str,
        schema_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT table_name,view_definition
                FROM information_schema.views
                WHERE table_schema = ?;";
            let result2: Vec<ValueMap> = rb
                .query_decode(_sql, vec![Value::String(schema_name.to_string())])
                .await
                .unwrap();
            // let mut db_struct = self.databases.lock().unwrap();

            Ok(result2)
        }
    }

    fn get_stored_procedures(
        &self,
        db_name: &str,
        schema_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async move {
            let rb = self.rbatis_connect(db_name).await?.unwrap();
            let _sql = "SELECT 
                                    n.nspname AS schema_name,
                                    p.proname AS procedure_name,
                                    pg_catalog.pg_get_function_identity_arguments(p.oid) AS arguments,
                                    pg_catalog.pg_get_function_result(p.oid) AS return_type,
                                    r.routine_definition AS procedure_body
                                FROM 
                                    pg_catalog.pg_proc p
                                JOIN 
                                    pg_catalog.pg_namespace n ON n.oid = p.pronamespace
                                JOIN 
                                    information_schema.routines r ON r.routine_name = p.proname 
                                                                AND r.specific_schema = n.nspname
                                WHERE 
                                    n.nspname = 'public' AND r.routine_type = 'PROCEDURE';";
            let result2: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
            Ok(result2)
        }
    }

    fn get_functions(
        &self,
        db_name: &str,
        schema_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT 
                                    n.nspname AS schema_name,
                                    p.proname AS function_name,
                                    pg_catalog.pg_get_function_identity_arguments(p.oid) AS arguments,
                                    pg_catalog.pg_get_function_result(p.oid) AS return_type,
                                    pg_catalog.pg_get_functiondef(p.oid) AS full_function,
                                    r.routine_definition AS function_body
                                FROM 
                                    pg_catalog.pg_proc p
                                JOIN 
                                    pg_catalog.pg_namespace n ON n.oid = p.pronamespace
                                JOIN 
                                    information_schema.routines r ON r.routine_name = p.proname 
                                                                AND r.specific_schema = n.nspname
                                WHERE 
                                    n.nspname = ? AND r.routine_type = 'FUNCTION' AND n.nspname NOT IN ('pg_catalog', 'information_schema');";
            let result2: Vec<ValueMap> = rb
                .query_decode(_sql, vec![Value::String(schema_name.to_string())])
                .await
                .unwrap();
            Ok(result2)
        }
    }

    fn get_trigger_functions(
        &self,
        db_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT 
                                tgname AS trigger_name,
                                proname AS function_name,
                                pg_get_functiondef(pg_proc.oid) AS function_definition
                            FROM 
                                pg_trigger
                            JOIN 
                                pg_proc ON pg_proc.oid = pg_trigger.tgfoid
                            WHERE 
                                NOT tgisinternal;";
            let result2: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

            Ok(result2)
        }
    }

    fn get_event_triggers(
        &self,
        db_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT evtname 
                FROM pg_event_trigger;";
            let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

            Ok(result)
        }
    }

    fn get_aggregates(
        &self,
        db_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT proname 
                FROM pg_proc 
                WHERE prokind='a';";
            let result2: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
            Ok(result2)
        }
    }

    fn get_materalized_views(
        &self,
        db_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();
            let _sql = "SELECT matviewname,definition
                FROM pg_matviews;";
            let result2: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
            Ok(result2)
        }
    }

    ///Get the types from the database if exists
    fn get_types(
        &self,
        db_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT typname AS type_name, 
                                    attname AS attribute_name, 
                                    format_type(atttypid, atttypmod) AS data_type
                                FROM pg_type                                     
                                JOIN pg_class ON pg_class.oid = pg_type.typrelid
                                JOIN pg_attribute ON pg_attribute.attrelid = pg_class.oid
                                JOIN pg_namespace ns ON ns.oid = pg_type.typnamespace
                                WHERE typtype = 'c' 
                                AND attnum > 0
                                AND ns.nspname NOT IN ('pg_catalog', 'information_schema')
                                AND pg_class.relkind = 'c'  -- Ensure it is a composite type, not a table
                                ORDER BY typname, attnum;";
            let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
            Ok(result)
        }
    }

    fn get_languages(
        &self,
        db_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
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
    }

    fn get_catalogs(
        &self,
        db_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT nspname AS catalog_name
                                FROM pg_namespace
                                WHERE nspname IN ('pg_catalog', 'information_schema')
                                ORDER BY nspname;";
            let result2: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

            Ok(result2)
        }
    }

    fn get_foreign_data_wrappers(
        &self,
        db_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT fdwname 
                FROM pg_foreign_data_wrapper;";
            let result2: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

            Ok(result2)
        }
    }

    //TODO SELECT table_name FROM information_schema.tables WHERE table_schema = '?'; catalogobjects(?)
    fn get_schemas(
        &self,
        db_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            /*  let _sql = "SELECT schema_name
            FROM information_schema.schemata
            WHERE schema_name NOT IN ('information_schema', 'pg_catalog');";*/
            let _sql = "SELECT schema_name 
                FROM information_schema.schemata";
            let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

            Ok(result)
        }
    }

    fn get_indexes(
        &self,
        db_name: &str,
        table_name: &str,
        schema_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT 
                        t.relname AS table_name,
                        i.relname AS index_name,
                        a.attname AS column_name,
                        idx.indisunique AS is_unique
                    FROM 
                        pg_index AS idx
                    JOIN 
                        pg_class AS t ON t.oid = idx.indrelid
                    JOIN 
                        pg_class AS i ON i.oid = idx.indexrelid
                    JOIN 
                        pg_attribute AS a ON a.attnum = ANY(idx.indkey) AND a.attrelid = t.oid
                    WHERE 
                        t.relkind = 'r'
                        AND t.relname = ?;";
            let result2: Vec<ValueMap> = rb
                .query_decode(_sql, vec![Value::String(table_name.to_string())])
                .await
                .unwrap();
            Ok(result2)
        }
    }

    fn get_constraints(
        &self,
        db_name: &str,
        table_name: &str,
        schema_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();
            /*log::info!("Searching constraint for schema: {:?} and table: {:?}",schema_name.to_string(),table_name.to_string());
              let exists_sql = format!("SELECT EXISTS                               
            kcu.constraint_name,
            kcu.table_name,
            kcu.column_name,
            tc.constraint_type
        FROM 
            information_schema.key_column_usage kcu
        JOIN 
            information_schema.table_constraints tc 
        ON 
            kcu.constraint_name = tc.constraint_name
        WHERE 
            tc.table_schema = '{}'
            AND tc.table_name = '{}';",schema_name.to_string(),table_name.to_string());
            let exists:Value = rb.query_decode(&exists_sql, vec![]).await?;
            log::info!("Constrainst exists? {:?}",exists);*/
           
            
            let _sql = "SELECT                                 
                                kcu.constraint_name,
                                kcu.table_name,
                                kcu.column_name,
                                tc.constraint_type
                            FROM 
                                information_schema.key_column_usage kcu
                            JOIN 
                                information_schema.table_constraints tc 
                            ON 
                                kcu.constraint_name = tc.constraint_name
                            WHERE 
                                tc.table_schema = ?
                                AND tc.table_name = ?;";
            let result2:Vec<ValueMap> = rb.query_decode(_sql, vec![Value::String(schema_name.to_string()),Value::String(table_name.to_string())]).await.unwrap();
            Ok(result2)
        }
    }

    fn get_sequences(
        &self,
        db_name: &str,
        schema_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();
            let _sql = "SELECT sequence_name,start_value,minimum_value,maximum_value,increment,cycle_option
                FROM information_schema.sequences 
                WHERE sequence_schema = ?;";
            let result2: Vec<ValueMap> = rb
                .query_decode(_sql, vec![Value::String(schema_name.to_string())])
                .await
                .unwrap();
            Ok(result2)
        }
    }

    fn get_roles_and_users(
        &self,
        db_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "
            SELECT
            rolname AS role_name,
            rolsuper AS is_superuser,
            rolinherit AS can_inherit,
            rolcreaterole AS can_create_roles,
            rolcreatedb AS can_create_db,
            rolcanlogin AS can_login,
            rolreplication AS can_replicate,
            rolconnlimit AS connection_limit,
            rolvaliduntil AS valid_until
        FROM
            pg_roles;";
            let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
            Ok(result)
        }
    }

    fn get_table_statistics(
        &self,
        db_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT relname, n_live_tup, n_dead_tup
                FROM pg_stat_user_tables;";
            let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
            Ok(result)
        }
    }

    fn get_active_sessions(
        &self,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            self.connect("postgres", self.base_url.as_str()).await;
            let rb = match self.rb_map.get("postgres") {
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql = "SELECT pid, usename, application_name, client_addr 
                FROM pg_stat_activity;";
            let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
            Ok(result)
        }
    }

    fn get_locks(
        &self,
        db_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT * FROM pg_locks;";
            let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
            Ok(result)
        }
    }

    fn get_partitions(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT
                                c.relname AS partition_name
                            FROM
                                pg_inherits AS i
                            JOIN
                                pg_class AS c ON c.oid = i.inhrelid
                            JOIN
                                pg_class AS p ON p.oid = i.inhparent
                            WHERE
                                p.relname = ?;";
            let result: Vec<ValueMap> = rb
                .query_decode(_sql, vec![Value::String(table_name.to_string())])
                .await
                .unwrap();
            Ok(result)
        }
    }

    fn get_user_privileges(
        &self,
        db_name: &str,
        user_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
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
    }

    fn get_database_settings(
        &self,
        db_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SHOW ALL;";
            let result: Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
            Ok(result)
        }
    }

    fn get_foreign_key_relationships(
        &self,
        db_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
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
    }

    fn get_triggers_associated_with_table(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT
                                    t.tgname AS trigger_name,
                                    c.relname AS table_name,
                                    pg_get_triggerdef(t.oid, true) AS trigger_definition
                                FROM
                                    pg_trigger t
                                JOIN
                                    pg_class c ON t.tgrelid = c.oid
                                JOIN
                                    pg_namespace n ON c.relnamespace = n.oid
                                WHERE
                                    n.nspname = ?
                                    AND c.relname = ?;";
            let result: Vec<ValueMap> = rb
                .query_decode(_sql, vec![Value::String("public".to_string()),Value::String(table_name.to_string())])
                .await
                .unwrap();
            Ok(result)
        }
    }

    fn get_default_columns_value(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
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
    }

    fn get_rls_policies(
        &self,
        db_name: &str,
        table_name: &str,
        schema_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT pol.polname AS policy_name,
                            pol.polcmd AS command,
                            pg_catalog.pg_get_expr(pol.polqual, pol.polrelid) AS policy_using,
                            pg_catalog.pg_get_expr(pol.polwithcheck, pol.polrelid) AS policy_with_check,
                            pol.polroles AS policy_roles
                        FROM pg_catalog.pg_policy pol
                        JOIN pg_catalog.pg_class tab ON tab.oid = pol.polrelid
                        JOIN pg_catalog.pg_namespace nsp ON nsp.oid = tab.relnamespace
                        WHERE tab.relname = ?
                        AND nsp.nspname = ?;";
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
    }

    fn get_rules(
        &self,
        db_name: &str,
        table_name: &str,
    ) -> impl Future<Output = Result<Vec<ValueMap>, rbdc::Error>> + Send {
        async {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT r.rulename AS rule_name,
                            pg_get_ruledef(r.oid) AS rule_definition
                            FROM pg_rewrite r                                                        
                            JOIN pg_class t ON r.ev_class = t.oid
                            WHERE t.relname = ?;";
            let result2: Vec<ValueMap> = rb
                .query_decode(_sql, vec![Value::String(table_name.to_string())])
                .await
                .unwrap();
            Ok(result2)
        }
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
    
    /// **Helper function to setup test database connection**
    async fn setup_test_db() -> PostgresRepository {
       // let rb = rbatis::RBatis::new();
        //rb.init(rbdc_pg::driver::PgDriver {}, "postgres://test_user:test_password@localhost/test_db")
          //  .unwrap();
        let connection_info:DatabaseConnection = DatabaseConnection{
            driver_type: "postgres".to_string(),
            username: "mzeteny".to_string(),
            password: "zetou123".to_string(),
            server: "localhost".to_string(),
            port: "5432".to_string(),
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
        rb.init(PgDriver {}, url.as_str());
        let mut repo = PostgresRepository::new(connection_info);
        repo.rb_map.insert("test_db".to_string(),rb.clone());
        let mut db_metadata = DatabaseMetadata {
            name: "test_db".to_string(),
            schemas: Some(HashMap::new()),
            foreign_data_wrappers: Some(HashMap::new()),
            catalogs: Some(HashMap::new()),
            type_: "database".to_string(),
        };
        let schemas = Schema {
            name: "public".to_string(),
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
            roles: Some(HashMap::new()),
            type_: Some("schema".to_string()),
        };
        let mut schema_hashmap = HashMap::new();
        schema_hashmap.insert("public".to_string(), schemas);
        let _ = db_metadata.schemas.insert(schema_hashmap);
        repo.databases.insert("test_db".to_string(), db_metadata);
        repo
    }

    #[tokio::test]
    async fn test_create_table_is_ok() {
        let repo = setup_test_db().await;
        let drop_query = "DROP TABLE IF EXISTS fasz;";
        repo.rb_map.get("test_db").unwrap().exec(drop_query, vec![]).await.unwrap();
        log::info!("Drop query: {}", drop_query);   
        // ✅ **Step 1: Define the table schema**   
      //  let test_table = CreateTableInfo {table_name:"test_table".to_string(),columns:vec![Column{name:"id".to_string(),data_type:Some("SERIAL".to_string()),is_primary_key:Some(true),is_nullable:Some(false),default_value:None,maximum_length:None, table_name: todo!(), db_name: todo!(), type_: todo!() },Column{name:"name".to_string(),data_type:Some("VARCHAR(255)".to_string()),is_primary_key:Some(false),is_nullable:Some(false),default_value:None,maximum_length:Some(255), table_name: todo!(), db_name: todo!(), type_: todo!() },], db_name: todo!() 
      //  };
      let table_t: CreateTableInfo = CreateTableInfo{table_name:"fasz".to_string(),columns:vec![Column{name:"namajo".to_string(),data_type:Some("INT".to_string()),is_nullable:Some(true),default_value:Some("".to_string()),is_primary_key:Some(false),maximum_length:Some(0),table_name:"test_t".to_string(),db_name:"test_db".to_string(),type_:"column".to_string(), schema_name: Some("public".to_string()) }],db_name:"test_db".to_string(), schema_name: "public".to_string() };

        println!("Test table: {:?}",table_t);
        let result = repo.create_table("test_db",&table_t).await;
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

        let table_t: CreateTableInfo = CreateTableInfo{table_name:"fasz".to_string(),columns:vec![Column{name:"namajo".to_string(),data_type:Some("INT".to_string()),is_nullable:Some(true),default_value:Some("".to_string()),is_primary_key:Some(false),maximum_length:Some(0),table_name:"test_t".to_string(),db_name:"test_db".to_string(),type_:"column".to_string(), schema_name: Some("public".to_string()) }],db_name:"akela".to_string(), schema_name: "public".to_string() };
        
        let result = repo.create_table("test_dab",&table_t).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_sequence_is_ok() {
        let repo = setup_test_db().await;
        let drop_query = format!("DROP SEQUENCE IF EXISTS test_sequa;");
        repo.rb_map.get("test_db").unwrap().exec(&drop_query, vec![]).await.unwrap();
        let table_t: CreateTableInfo = CreateTableInfo{table_name:"faszaa".to_string(),columns:vec![Column{name:"namajo".to_string(),data_type:Some("INT".to_string()),is_nullable:Some(true),default_value:Some("".to_string()),is_primary_key:Some(false),maximum_length:Some(0),table_name:"test_t".to_string(),db_name:"test_db".to_string(),type_:"column".to_string(), schema_name: Some("public".to_string()) }],db_name:"test_db".to_string(), schema_name: "public".to_string() };
        let seq_info = CreateSequenceInfo {sequence_name:"test_sequa".to_string(),increment:"1".to_string(),minimum_val:"1".to_string(),maximum_val:"100".to_string(),start_val:"1".to_string(),cycle:true, schema_name: "public".to_string() };
        let _ = repo.create_table("test_db",&table_t).await;  
        let result = repo.create_sequence("faszaa", "test_db","public", &seq_info).await;
        println!("result: {:?}", result);
        assert!(result.is_ok());

        let drop_query_t = "DROP TABLE IF EXISTS faszaa;";
        repo.rb_map.get("test_db").unwrap().exec(drop_query_t, vec![]).await.unwrap();
        let drop_query = "DROP SEQUENCE IF EXISTS test_sequa;";
        repo.rb_map.get("test_db").unwrap().exec(drop_query, vec![]).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_sequence_table_not_exist(){
        let repo = setup_test_db().await;
        let seq_info = CreateSequenceInfo {sequence_name:"test_sequa".to_string(),increment:"1".to_string(),minimum_val:"1".to_string(),maximum_val:"100".to_string(),start_val:"100".to_string(),cycle:false, schema_name: todo!() };

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
        let drop_query_t = "DROP TABLE IF EXISTS test_table;";
        repo.rb_map.get("test_db").unwrap().exec(drop_query_t, vec![]).await.unwrap();
        let drop_query = "DROP INDEX IF EXISTS test_idxq;";
        repo.rb_map.get("test_db").unwrap().exec(drop_query, vec![]).await.unwrap();
        let table_t: CreateTableInfo = CreateTableInfo{table_name:"test_table".to_string(),columns: vec![Column{name:"id".to_string(),data_type:Some("INT".to_string()),is_nullable:Some(true),default_value:Some("".to_string()),is_primary_key:Some(false),maximum_length:Some(0),table_name:"test_t".to_string(),db_name:"test_db".to_string(),type_:"column".to_string(), schema_name: todo!() }],db_name:"test_db".to_string(), schema_name: "public".to_string() };

        let index_info = CreateIndexInfo {
            index_name: "test_idxq".to_string(),
            table_name: "test_table".to_string(),
            columns: vec!["id".to_string()],
            schema_name: "public".to_string(),
        };

        let _ = repo.create_table("test_db",&table_t).await;  
        let result = repo.create_index(&index_info, "test_db").await;
        
        println!("Is error? {}",result.is_err());
        println!("Result: {:?}",result);
        
        assert!(result.is_ok());

        let drop_query_t = "DROP TABLE IF EXISTS test_table;";
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
            schema_name: "public".to_string(),
        };
        let result = repo.create_index(&index_info, "test_db").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_view_is_ok() {
        let repo = setup_test_db().await;
        let table_t: CreateTableInfo = CreateTableInfo{table_name:"testablae".to_string(),columns:vec![Column{name:"id".to_string(),data_type:Some("INT".to_string()),is_nullable:Some(true),default_value:Some("".to_string()),is_primary_key:Some(false),maximum_length:Some(0),table_name:"test_t".to_string(),db_name:"test_db".to_string(),type_:"column".to_string(), schema_name: todo!() }],db_name:"test_db".to_string(), schema_name: todo!() };

        let view_info = CreateViewInfo {view_name:"test_view".to_string(),stmt:"SELECT * FROM testablae;".to_string(),columns:vec!["id".to_string()],table_name:"testablae".to_string(), schema_name: todo!() };
        let _ = repo.create_table("test_db",&table_t).await;  

        let result = repo.create_view(&view_info,"test_db","public").await;
        println!("result: {:?}",result);
        assert!(result.is_ok());

        let drop_query_t = "DROP TABLE IF EXISTS testablaa;";
        repo.rb_map.get("test_db").unwrap().exec(drop_query_t, vec![]).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_view_err_table_not_exist(){
        let repo = setup_test_db().await;
        let view_info = CreateViewInfo {view_name:"test_view".to_string(),stmt:"SELECT * FROM testablae;".to_string(), columns:vec!["id".to_string()], table_name: "testablae".to_string(), schema_name: "public".to_string()};
        let result = repo.create_view(&view_info,"test_db","public").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_create_role_is_ok(){
        let repo = setup_test_db().await;

        let role_info:Role = Role { name: "test_rolee".to_string(), is_super: Some(false), is_insherit: Some(false), is_create_role: Some(false), is_create_db: Some(true), can_login: Some(true), is_replication: Some(false), connection_limit: Some(100), valid_until: Some("".to_string()), password: Some("valamike".to_string()), db_name: "test_db".to_string(), type_: "role".to_string(), schema_name: "public".to_string() };
        let result = repo.create_role(role_info, "test_db","public").await.await;

        println!("result: {:?}",result);
        assert!(result.is_ok());

        let drop_query_t = "DROP ROLE IF EXISTS test_role;";
        repo.rb_map.get("test_db").unwrap().exec(drop_query_t, vec![]).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_role_err_database_not_exist(){
        let repo = setup_test_db().await;

        let role_info:Role = Role { name: "test_rolee".to_string(), is_super: Some(false), is_insherit: Some(false), is_create_role: Some(false), is_create_db: Some(true), can_login: Some(true), is_replication: Some(false), connection_limit: Some(100), valid_until: Some("".to_string()), password: Some("valamike".to_string()), db_name: "test_db".to_string(), type_: "role".to_string(), schema_name: "public".to_string() };
        let result = repo.create_role(role_info, "akela_db","public").await.await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_view_is_ok(){
        let repo = setup_test_db().await;

        //let table_t: CreateTableInfo = CreateTableInfo{table_name:"testablae".to_string(),columns: vec![Column{ name: "id".to_string(), data_type: Some("INT".to_string()), is_nullable: Some(true), default_value: Some("".to_string()), is_primary_key: Some(false), maximum_length: Some(0), table_name: "test_t".to_string(), db_name: "test_db".to_string(), type_: "column".to_string() }],db_name:"test_db".to_string()};
        let view_info = CreateViewInfo {view_name:"test_view".to_string(),stmt:"SELECT * FROM testablae;".to_string(), columns:vec!["id".to_string()], table_name: "testablae".to_string(),schema_name: "public".to_string()};
        let old_view_info = View {name:"test_view".to_string(),definition:"SELECT * FROM testablae;".to_string(),type_:"view".to_string(), schema_name : "public".to_string(), db_name: "test_db".to_string()};
        let new_view_info = View {name:"test_view22".to_string(),definition:"SELECT id FROM testablae;".to_string(),type_:"view".to_string(), schema_name: "public".to_string(), db_name: "test_db".to_string()};
        let _ = repo.create_view(&view_info,"test_db","public").await;

        let result = repo.edit_view(old_view_info, new_view_info, "test_db").await.await;

        println!("result: {:?}",result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_edit_view_err_old_view_not_exists(){
        let repo = setup_test_db().await;

        let old_view_info = View {name:"test_view".to_string(),definition:"SELECT * FROM testablae;".to_string(),type_:"view".to_string(), schema_name: "public".to_string(), db_name: "test_db".to_string()};
        let new_view_info = View {name:"test_view22".to_string(),definition:"SELECT id FROM testablae;".to_string(),type_:"view".to_string(), schema_name: "public".to_string(), db_name: "test_db".to_string()};

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
            schema_name: "public".to_string(),
        };
        let old_index_info = Index {
            table_name: Some("test_tabletete".to_string()),
            name: "test_idxq".to_string(),
            definition: Some("".to_string()),
            column_name: Some(vec!["id".to_string()]),
            non_unique: Some(false),
            db_name: "test_db".to_string(),
            type_: "index".to_string(),
            schema_name: Some("public".to_string()),
        };
        let new_index_info = Index {
            table_name: Some("test_tabletete".to_string()),
            name: "test_idxqcq".to_string(),
            definition: Some("".to_string()),
            column_name: Some(vec!["id".to_string()]),
            non_unique: Some(false),
            db_name: "test_db".to_string(),
            type_: "index".to_string(),
            schema_name: Some("public".to_string()),
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
            schema_name: "public".to_string(),
        };
        let old_index_info = Index {
            table_name: Some("test_tabletete".to_string()),
            name: "test_idxq".to_string(),
            definition: Some("".to_string()),
            column_name: Some(vec!["id".to_string()]),
            non_unique: Some(false),
            db_name: "test_db".to_string(),
            type_: "index".to_string(),
            schema_name: Some("public".to_string()),
        };

        let new_index_info = Index {
            table_name: Some("test_tabletete".to_string()),
            name: "test_idxqcq".to_string(),
            definition: Some("".to_string()),
            column_name: Some(vec!["id".to_string()]),
            non_unique: Some(false),
            db_name: "test_db".to_string(),
            type_: "index".to_string(),
            schema_name: Some("public".to_string()),
        };
        let _ = repo.create_index(&create_index_info, "test_db");
        let result = repo.edit_index("test_tabletete", new_index_info, old_index_info, "test_db").await.await;
    
        assert!(result.is_err());
    }


    #[tokio::test]
    async fn test_edit_sequence_is_ok(){
        let repo = setup_test_db().await;
        let table_t: CreateTableInfo = CreateTableInfo{table_name:"faszaa".to_string(),columns: vec![Column{name:"namajo".to_string(),data_type:Some("INT".to_string()),is_nullable:Some(true),default_value:Some("".to_string()),is_primary_key:Some(false),maximum_length:Some(0),table_name:"test_t".to_string(),db_name:"test_db".to_string(),type_:"column".to_string(), schema_name: todo!() }],db_name:"test_db".to_string(), schema_name: "public".to_string()};
        let seq_info = CreateSequenceInfo {
            sequence_name: "test_sequa".to_string(),
            increment: "1".to_string(),
            minimum_val: "1".to_string(),
            maximum_val: "100".to_string(),
            start_val: "1".to_string(),
            cycle: true,
            schema_name: "public".to_string(),
        };
        let _ = repo.create_table("test_db",&table_t).await;  
        let _ = repo.create_sequence("faszaa", "test_db", "public",&seq_info).await;


        let old_seq_info = Sequence {
            name: "test_sequa".to_string(),
            increment: Some("1".to_string()),
            minimum_val: Some("1".to_string()),
            maximum_val: Some("100".to_string()),
            start_val: Some("1".to_string()),
            cycle: Some("CYCLE".to_string()),
            db_name: "test_db".to_string(),
            type_: Some("sequence".to_string()),
            schema_name: Some("public".to_string()),
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
            schema_name: Some("public".to_string()),
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
            schema_name: Some("public".to_string()),
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
            schema_name: Some("public".to_string()),
        };
        let result = repo.edit_sequence(old_seq_info, new_seq, "test_db").await.await;
        assert!(result.is_err());
    }
}