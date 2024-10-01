    use crate::metadata::column::Column;
use crate::metadata::table;
use crate::metadata::{function::Function, materalized_view::MateralizedView, procedure::Procedure, table:: Table, trigger, view::View};
    use crate::metadata::rls_policy::RlsPolicy;
    use crate::metadata::rule::Rule;
    use crate::metadata::constraint::Constraint;
    use crate::metadata::sequence::Sequence;
    use crate::metadata::aggregate::Aggregate;
    use crate::metadata::catalog::Catalog;
    use crate::repository::database_repository::DatabaseRepository;
    use column::PgColumn;
    use dashmap::mapref::one::Ref;
    use fast_pool::Pool;
    use rbatis::{executor::RBatisRef, DefaultPool};
    use rbdc::db::{self, ConnectOptions};
    use rbdc_pg::connection::PgConnection;
    use rbdc_pg::*;
    use rbs::to_value;
    use rbs::value::map::ValueMap;
    use serde_json::Deserializer;
    use std::collections::HashMap;
    use std::{borrow::Borrow, future::Future, ops::Deref, result, sync::Mutex};
    use rbdc::Error;
    use rbs::Value;
    use serde::{Serialize,Deserialize};
    use std::sync::Arc;
    use log::{error, info, warn};
    use fast_log::init;
    use crate::metadata::database::Database;
    use dashmap::DashMap;
    use crate::DatabaseConnection;
    #[derive(Debug,Clone)]
    pub struct PostgresRepository{
        rb_map:DashMap<String,Arc<rbatis::RBatis>>,
        base_url:String,
        pub databases:DashMap<String,Database>,
        connection_info:DatabaseConnection,
    }

    impl PostgresRepository{
        pub fn new(connection_info:DatabaseConnection) -> Self{
            let rb_map = DashMap::new();
            let databases = DashMap::new();
            let base_url = String::from(format!("{}://{}:{}@{}:{}/postgres",connection_info.driver_type,
                                            connection_info.username,connection_info.password,
                                            connection_info.server,connection_info.port));
            return PostgresRepository{ rb_map, base_url, databases, connection_info};
        }

        ///Add the database to the pool if not exists
        ///It's create a new rbatis, initialize it add add to the pool
        async fn connect(&self,db_name:&str,url:&str) -> Result<(), Box<dyn std::error::Error>> {
            if !self.rb_map.contains_key(db_name){
                log::info!("new pool adding... database: {:?}",db_name);
                let rb = Arc::new(rbatis::RBatis::new());
                match rb.init(PgDriver {}, url) {
                    Ok(_) => log::info!("Connection to {} successful", db_name),
                    Err(e) => {
                        log::error!("Failed to initialize rbatis for {}: {:?}", db_name, e);
                        return Err(Box::new(e)); // Return the error early
                    }
                }
                self.rb_map.insert(db_name.to_string(),rb);
                let db_node = Database{
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
                };
                self.databases.insert(db_name.to_string(), db_node);
                log::info!("Database node created for {}", db_name);
            }
            Ok(())
        }

        ///Get the database by its name
        async fn get_database_(&self,db_name:&str)->std::option::Option<dashmap::mapref::one::Ref<'_, std::string::String, Database, >>{
            self.databases.get(db_name)
        }

        ///connect to rbatis if it isnt cached
        async fn rbatis_connect(&self,db_name:&str)->Result<Option<Ref<'_,String,Arc<rbatis::RBatis>>>,rbdc::Error>{
            let cached_rb = self.rb_map.get(db_name);
            if cached_rb.is_some(){
                log::info!("rb cached");
                return Ok(cached_rb);
            }
            log::info!("rb isnt cached");          
            let url = String::from(format!("{}://{}:{}@{}:{}/{}",
                            self.connection_info.driver_type,self.connection_info.username,
                            self.connection_info.password,self.connection_info.server,
                            self.connection_info.port,db_name));
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
                    Err(rbdc::Error::from("Database not found after connection attempt"))
                }
            }
            //Ok(rb)
        }
    }


    impl DatabaseRepository for PostgresRepository{

        ///Get the attached databases for the sqlite file
        async fn get_databases(&self)-> Result<Value,rbdc::Error> {
            log::info!("PgRepository: Get databases");
            let rb = self.rbatis_connect("postgres").await?.unwrap();
            let _sql = "SELECT datname 
            FROM pg_database 
            WHERE datistemplate=false;";
            println!("rb: {:?}",rb);
            let result = rb.query(_sql,vec![]).await.unwrap();
            let t_result:Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
            
            for res in &t_result{
                let db_name = res.0.get(&Value::String("datname".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                if !self.rb_map.contains_key(db_name){
                    let rb = Arc::new(rbatis::RBatis::new());
                    self.rb_map.insert(db_name.to_string(),rb);

                            let db_node = Database{
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
                            };
                            self.databases.insert(db_name.to_string(), db_node);
                }
            }

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
            Ok(result)
        }

        ///Get all tables in the database
        async fn get_tables(&self, db_name:&str)-> Result<Value,rbdc::Error> {
            log::info!("Get tables...");
            let rb = self.rbatis_connect(db_name).await?.unwrap();
            let _sql = "SELECT table_name
            FROM information_schema.tables
            WHERE table_schema = 'public';";
            let result = rb.query(_sql,vec![]).await.unwrap();
            let result_2:Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
            if !result_2.is_empty(){
                if let Some(mut node) = self.databases.get_mut(db_name){
                    let table_map = node.value_mut().tables.get_or_insert_with(HashMap::new);
                    for table in &result_2{
                        let mut table_name = table.0.get(&Value::String("table_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                        let tb_node = Table{
                            name : table_name.to_string(), 
                            columns : Some(HashMap::new()),
                            constraints: Some(HashMap::new()),
                            indexes: Some(HashMap::new()),
                            triggers: Some(HashMap::new()),
                            rules: Some(HashMap::new()),
                            rls_policies: Some(HashMap::new()),
                        };
                        table_map.insert(table_name.to_string(), tb_node);
                    }
                }else{

                }
            }else{
                return Ok(Value::Null);
            }
            Ok(result)
        }

        ///Get all columns in the table
        async fn get_columns(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();
 
            let _sql = "SELECT column_name,data_type,is_nullable,column_default
            FROM information_schema.columns
            WHERE table_name = ?;";
            
            let result = rb.query(_sql, vec![Value::String(table_name.to_string())]).await.unwrap();
            let result_2:Vec<ValueMap> = rb.query_decode(_sql, vec![Value::String(table_name.to_string())]).await.unwrap();
            println!("DECODE RESULTS");
        
                if let Some(mut node) = self.databases.get_mut(db_name){
                    if let Some(table_map) = node.value_mut().tables.clone().unwrap().get_mut(db_name){
                        let columns_map = table_map.columns.get_or_insert_with(HashMap::new);
                        for col in &result_2{
                                let col_name = col.0.get(&Value::String("column_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                                let data_type = col.0.get(&Value::String("data_type".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                                let is_nullable = col.0.get(&Value::String("is_nullable".to_string())).and_then(|v| v.as_bool()).unwrap_or_default();
                                let column_default = col.0.get(&Value::String("column_default".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                                
                                let _col_node = crate::metadata::column::Column{
                                    name : String::from(col_name),//col_name.1.to_string(),
                                    data_type: Some(String::from(data_type)),     
                                    is_nullable: Some(is_nullable),
                                    default_value: Some(String::from(column_default)),
                                };
                                
                                columns_map.insert(col_name.to_string(), _col_node);
                        }
                    }
                }
            Ok(result)
        }

        async fn get_views(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();
            
            let _sql = "SELECT table_name,view_definition
            FROM information_schema.views
            WHERE table_schema = 'public';";
            let result = rb.query(_sql, vec![]).await.unwrap();
            let result2:Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
               // let mut db_struct = self.databases.lock().unwrap();
                if let Some(mut node) = self.databases.get_mut(db_name){
                    let view_map = node.value_mut().views.get_or_insert_with(HashMap::new);
                    for view in &result2{
                            let view_name = view.0.get(&Value::String("table_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let view_definition = view.0.get(&Value::String("view_definition".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let view_node = View{
                                name : view_name.to_string(),
                                definition : "View ".to_string(),
                            };
                            view_map.insert(view_name.to_string(), view_node);
                        
                    }
                }else{
                    log::info!("Node is not OK");
                }

            Ok(result)
        }

        async fn get_stored_procedures(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT routine_name
            FROM information_schema.routines
            WHERE routine_type = 'PROCEDURE' AND specific_schema = 'public';";
            let result = rb.query(_sql, vec![]).await.unwrap();
            let result2:Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

                //let mut db_struct = self.databases.lock().unwrap();
                if let Some(mut node) = self.databases.get_mut(db_name){
                    let stored_procedure_map = node.value_mut().procedures.get_or_insert_with(HashMap::new);
                    for stored_procedure in &result2{
                            let stored_procedure_name = stored_procedure.0.get(&Value::String("routine_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let stored_procedure_node = Procedure{
                                name : stored_procedure_name.to_string(),
                                definition : "Stored Procedure ".to_string(),
                            };
                            stored_procedure_map.insert(stored_procedure_name.to_string(), stored_procedure_node);
                        
                    }
                }
            Ok(result)
        }

        async fn get_functions(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT
                n.nspname AS function_schema,
                p.proname AS function_name
            FROM
                pg_proc p
                LEFT JOIN pg_namespace n ON p.pronamespace = n.oid
            WHERE
                n.nspname NOT IN ('pg_catalog', 'information_schema')
            ORDER BY
                function_schema,
                function_name;";
            let result = rb.query(_sql, vec![]).await.unwrap();
            let result2:Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

                //let mut db_struct = self.databases.lock().unwrap();
                if let Some(mut node) = self.databases.get_mut(db_name){
                    let function_map = node.value_mut().functions.get_or_insert_with(HashMap::new);
                    for function in &result2{
                            let function_name = function.0.get(&Value::String("function_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let function_schema = function.0.get(&Value::String("function_schema".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let function_node = Function{
                                name : function_name.to_string(),
                                definition : function_schema.to_string(),
                            };
                        
                    }
                }else{
                    log::info!("Node is not OK");
                }
            
            Ok(result)
        }

        async fn get_trigger_functions(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql =  "SELECT tgname 
            FROM pg_trigger;";
            let result: Value = rb.query(_sql, vec![]).await.unwrap();
            let result2:Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
                //let mut db_struct = self.databases.lock().unwrap();
                if let Some(mut node) = self.databases.get_mut(db_name){
                    let trigger_map = node.value_mut().triggers.get_or_insert_with(HashMap::new);
                    for trigger in &result2{
                            let trigger_name = trigger.0.get(&Value::String("tgname".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let trigger_node = trigger::Trigger{
                                name : trigger_name.to_string(),
                            };
                            trigger_map.insert(trigger_name.to_string(), trigger_node);
                        
                    }
                }else{
                    log::info!("Node is not OK");
                }
            Ok(result)
        }

        async fn get_event_triggers(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT evtname 
            FROM pg_event_trigger;";
            let result = rb.query(_sql, vec![]).await.unwrap();

            Ok(result)

        }

        async fn get_aggregates(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT proname 
            FROM pg_proc 
            WHERE prokind='a';";
            let result = rb.query(_sql, vec![]).await.unwrap();
            let result2:Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
               // let mut db_struct = self.databases.lock().unwrap();
                if let Some(mut node) = self.databases.get_mut(db_name){
                    let aggregate_map = node.value_mut().aggregates.get_or_insert_with(HashMap::new);
                    for aggregate in &result2{
                            let aggregate_name = aggregate.0.get(&Value::String("proname".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let aggregate_node = Aggregate{
                                name : aggregate_name.to_string(),
                            };
                            aggregate_map.insert(aggregate_name.to_string(), aggregate_node);
                        
                    }
                }
            
            Ok(result)
        }

        async fn get_materalized_views(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();
            let _sql = "SELECT matviewname,definition
            FROM pg_matviews;";
            let result = rb.query(_sql, vec![]).await.unwrap();
            let result2:Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

                if let Some(mut node) = self.databases.get_mut(db_name){
                    let matview_map = node.value_mut().materalized_views.get_or_insert_with(HashMap::new);
                    for matview in &result2{
                            let matview_name = matview.0.get(&Value::String("matviewname".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let matview_def = matview.0.get(&Value::String("definition".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let matview_node = MateralizedView{
                                name : matview_name.to_string(),
                                definition : matview_def.to_string(),
                            };
                            matview_map.insert(matview_name.to_string(), matview_node);
                        
                    }
                }else{
                    log::info!("Node is not OK");
                }
            
            Ok(result)
        }

        ///Get the types from the database if exists
        async fn get_types(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT typname 
            FROM pg_type 
            WHERE typtype = 'b';";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)
        }

        async fn get_languages(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://mzeteny:zetou123@localhost:5432/".to_string()+db_name;
            let _ = self.connect(db_name,url.as_str()).await;
            let rb = match self.rb_map.get(db_name){
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql ="SELECT lanname 
            FROM pg_language;";
            let result = rb.query(_sql, vec![]).await.unwrap();
            
            Ok(result)

        }

        async fn get_catalogs(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT nspname AS catalog_name
                            FROM pg_namespace
                            WHERE nspname IN ('pg_catalog', 'information_schema')
                            ORDER BY nspname;";
            let result = rb.query(_sql, vec![]).await.unwrap();
            let result2:Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
               // let mut db_struct = self.databases.lock().unwrap();
                if let Some(mut node) = self.databases.get_mut(db_name){
                    let catalog_map = node.value_mut().catalogs.get_or_insert_with(HashMap::new);
                    for catalog in &result2{
                            let catalog_name = catalog.0.get(&Value::String("catalog_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let catalog_node = Catalog{
                                name : catalog_name.to_string(),
                            };
                            
                            catalog_map.insert(catalog_name.to_string(), catalog_node);
                    }
                }else{
                    log::info!("Node is not OK");
                }
            Ok(result)
        }

        async fn get_foreign_data_wrappers(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT fdwname 
            FROM pg_foreign_data_wrapper;";
            let result = rb.query(_sql, vec![]).await.unwrap();
            let result2:Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

                //let mut db_struct = self.databases.lock().unwrap();
                if let Some(mut node) = self.databases.get_mut(db_name){
                    let foreign_data_wrapper_map = node.value_mut().foreign_data_wrappers.get_or_insert_with(HashMap::new);
                    for foreign_data_wrapper in &result2{
                            let foreign_data_wrapper_name = foreign_data_wrapper.0.get(&Value::String("fdwname".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            
                            let foreign_data_wrapper_node = crate::metadata::foreign_data_wrapper::ForeignDataWrapper{
                                name : foreign_data_wrapper_name.to_string(),
                            };
                            
                            foreign_data_wrapper_map.insert(foreign_data_wrapper_name.to_string(), foreign_data_wrapper_node);
                        
                    }
                }else{
                    log::info!("Node is not OK");
                }
            Ok(result)
        }

        //TODO SELECT table_name FROM information_schema.tables WHERE table_schema = '?'; catalogobjects(?)
        async fn get_schemas(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT schema_name 
            FROM information_schema.schemata
            WHERE schema_name='public';";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)
        }

        async fn get_indexes(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT indexname, indexdef
            FROM pg_indexes 
            WHERE schemaname = 'public';";
            let result = rb.query(_sql, vec![]).await.unwrap();
            let result2:Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
            
                if let Some(mut node) = self.databases.get_mut(db_name){
                    if let Some(table_map) = node.value_mut().tables.clone().unwrap().get_mut(db_name){
                        let index_map = table_map.indexes.get_or_insert_with(HashMap::new);
                        for index in &result2{
                                let index_name = index.0.get(&Value::String("indexname".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                                let index_definition = index.0.get(&Value::String("indexdef".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                                
                                let index_node = crate::metadata::index::Index{
                                    name : String::from(index_name),
                                    definition: Some(String::from(index_definition)),     
                                };
                                index_map.insert(index_name.to_string(), index_node);
                            
                        }
                    }
                }else{
                    log::info!("Node is not OK");
                }
            

            Ok(result)
        }

        async fn get_constraints(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT constraint_name, constraint_type
            FROM information_schema.table_constraints 
            WHERE table_schema = 'public';";
            let result = rb.query(_sql, vec![]).await.unwrap();
            let result2:Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();
                if let Some(mut node) = self.databases.get_mut(db_name){
                    if let Some(table_map) = node.value_mut().tables.clone().unwrap().get_mut(table_name){
                        let constraint_map = table_map.constraints.get_or_insert_with(HashMap::new);
                        for constraint in &result2{
                                let constraint_name = constraint.0.get(&Value::String("constraint_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                                let constraint_type = constraint.0.get(&Value::String("constraint_type".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                                
                                let constraint_node = crate::metadata::constraint::Constraint{
                                    name: String::from(constraint_name),
                                    c_type: String::from(constraint_type)
                                };
                                constraint_map.insert(constraint_name.to_string(), constraint_node);
                            
                        }
                    }
                    
                }else{
                    log::info!("Node is not OK");
                }
            
            Ok(result)

        }

        async fn get_sequences(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT sequence_name 
            FROM information_schema.sequences 
            WHERE sequence_schema = 'public';";
            let result = rb.query(_sql, vec![]).await.unwrap();
            let result2:Vec<ValueMap> = rb.query_decode(_sql, vec![]).await.unwrap();

                //let mut db_struct = self.databases.lock().unwrap();
                if let Some(mut node) = self.databases.get_mut(db_name){
                    let sequence_map = node.value_mut().sequences.get_or_insert_with(HashMap::new);
                    for seq in &result2{
                            let sequence_name = seq.0.get(&Value::String("sequence_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let sequence_node = Sequence{
                                name : sequence_name.to_string(),
                            };
                            sequence_map.insert(sequence_name.to_string(), sequence_node);
                        
                    }
                }else{
                    log::info!("Node is not OK");
                }

            Ok(result)

        }

        async fn get_roles_and_users(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT rolname FROM pg_roles;";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)

        }

        async fn get_table_statistics(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT relname, n_live_tup, n_dead_tup
            FROM pg_stat_user_tables;";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)

        }

        async fn get_active_sessions(&self)-> Result<Value,rbdc::Error> {
            self.connect("postgres",self.base_url.as_str()).await;
            let rb = match self.rb_map.get("postgres"){
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql = "SELECT pid, usename, application_name, client_addr 
            FROM pg_stat_activity;";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)

        }

        async fn get_locks(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT * FROM pg_locks;";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)

        }

        async fn get_partitions(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
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
            let result = rb.query(_sql, vec![Value::String(table_name.to_string())]).await.unwrap();
            Ok(result)

        }

        async fn get_user_privileges(&self,db_name:&str,user_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT grantee, privilege_type, table_name 
            FROM information_schema.role_table_grants 
            WHERE grantee = ?;";
            let result = rb.query(_sql, vec![Value::String(user_name.to_string())]).await.unwrap();
            Ok(result)
        }

        async fn get_database_settings(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SHOW ALL;";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)
        }

        async fn get_foreign_key_relationships(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT conname AS constraint_name, conrelid::regclass AS table_name,
        a.attname AS column_name, confrelid::regclass AS foreign_table_name,
        af.attname AS foreign_column_name
            FROM   pg_constraint
            JOIN   pg_attribute a ON a.attnum = ANY(conkey) AND a.attrelid = conrelid
            JOIN   pg_attribute af ON af.attnum = ANY(confkey) AND af.attrelid = confrelid
            WHERE  contype = 'f';";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)
        }

        async fn get_triggers_associated_with_table(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT tgname
            FROM pg_trigger
            WHERE tgrelid = ?::regclass;";
            let result = rb.query(_sql, vec![Value::String(table_name.to_string())]).await.unwrap();
            Ok(result)
        }

        async fn get_default_columns_value(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT column_name, column_default
            FROM information_schema.columns
            WHERE table_name = ?;";
            let result = rb.query(_sql, vec![Value::String(table_name.to_string())]).await.unwrap();
            Ok(result)

        }

        async fn get_rls_policies(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT pol.polname AS policy_name,
                        pol.polcmd AS command,
                        pg_catalog.pg_get_expr(pol.polqual, pol.polrelid) AS policy_using,
                        pg_catalog.pg_get_expr(pol.polwithcheck, pol.polrelid) AS policy_with_check,
                        pol.polroles AS policy_roles
                    FROM pg_catalog.pg_policy pol
                    JOIN pg_catalog.pg_class tab ON tab.oid = pol.polrelid
                    JOIN pg_catalog.pg_namespace nsp ON nsp.oid = tab.relnamespace
                    WHERE tab.relname = '?'
                    AND nsp.nspname = 'public';";
            let result = rb.query(_sql, vec![Value::String(table_name.to_string())]).await.unwrap();
            let result2:Vec<ValueMap> = rb.query_decode(_sql, vec![Value::String(table_name.to_string())]).await.unwrap();

                if let Some(mut node) = self.databases.get_mut(db_name){
                    if let Some(table_map) = node.value_mut().tables.clone().unwrap().get_mut(table_name){
                        let policy_map = table_map.rls_policies.get_or_insert_with(HashMap::new);
                        for policy in &result2{
                                let policy_name = policy.0.get(&Value::String("policy_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                                let policy_node = RlsPolicy{
                                    name : policy_name.to_string(),
                                    command : policy.0.get(&Value::String("command".to_string())).and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                                };
                                policy_map.insert(policy_name.to_string(), policy_node);
                            
                        }
                    }
                }else{
                    log::info!("Node is not OK");
                }
            Ok(result)

        }

        async fn get_rules(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT r.rulename AS rule_name,
                        pg_get_ruledef(r.oid) AS rule_definition
                        FROM pg_rewrite r                                                        
                        JOIN pg_class t ON r.ev_class = t.oid
                        WHERE t.relname = 'product';";
            let result = rb.query(_sql, vec![Value::String(table_name.to_string())]).await.unwrap();
            let result2:Vec<ValueMap> = rb.query_decode(_sql, vec![Value::String(table_name.to_string())]).await.unwrap();
            
                if let Some(mut node) = self.databases.get_mut(db_name){
                    if let Some(table_map) = node.value_mut().tables.clone().unwrap().get_mut(table_name){
                        let rule_map = table_map.rules.get_or_insert_with(HashMap::new);//table_map.unwrap().get(table_name).unwrap().rules.get_or_insert_with(HashMap::new);//table_map.as_mut().unwrap().get(table_name).unwrap().rules.get_or_insert_with(HashMap::new());
                        for rule in &result2{
                                let rule_name = rule.0.get(&Value::String("rule_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                                let rule_node = Rule{
                                    name : rule_name.to_string(),
                                    definition : rule.0.get(&Value::String("rule_definition".to_string())).and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                                };
                                rule_map.insert(rule_name.to_string(), rule_node);
                            
                        }
                    }
                }else{
                    log::info!("Node is not OK");
                }
            Ok(result)

        }
        
    }

