    use crate::metadata::table;
use crate::metadata::{function::Function, materalized_view::MateralizedView, procedure::Procedure, repository::DatabaseRepository, table:: Table, trigger, view::View};
    use crate::metadata::rls_policy::RlsPolicy;
    use crate::metadata::rule::Rule;
    use crate::metadata::constraint::Constraint;
    use crate::metadata::sequence::Sequence;
    use crate::metadata::aggregate::Aggregate;
    use crate::metadata::catalog::Catalog;
    use fast_pool::Pool;
    use rbatis::{executor::RBatisRef, DefaultPool};
    use rbdc::db::{self, ConnectOptions};
    use rbdc_mssql::tiberius::Column;
    use rbdc_pg::connection::PgConnection;
    use rbdc_pg::*;
    use rbs::to_value;
    use std::collections::HashMap;
    use std::{borrow::Borrow, collections::DashMap, future::Future, ops::Deref, result, sync::Mutex};
    use rbdc::Error;
    use rbs::Value;
    use serde::{Serialize,Deserialize};
    use std::sync::Arc;
    use log::{error, info, warn};
    use fast_log::init;
    use crate::metadata::database::Database;
    use dashmap::DashMap;
    #[derive(Debug)]
    pub struct PostgresRepository{
        rb_map:DashMap<String,Arc<rbatis::RBatis>>,
        base_url:String,
        pub databases:DashMap<String,Database>,
    }

    impl PostgresRepository{
        pub fn new() -> Self{
            let rb_map = DashMap::new();
            let databases = DashMap::new();
            let base_url = String::from("postgresql://mzeteny:zetou123@localhost:5432/postgres");
            return PostgresRepository{ rb_map, base_url, databases};
        }

        ///Add the database to the pool if not exists
        ///It's create a new rbatis, initialize it add add to the pool
        async fn connect(&self,db_name:&str,url:&str) -> Result<(), Box<dyn std::error::Error>> {
            if !rb_map.contains_key(db_name){
                log::info!("new pool adding... database: {:?}",db_name);
                let rb = Arc::new(rbatis::RBatis::new());
                let _ = rb.init(PgDriver {}, url);
           
                self.rb_map.insert(db_name.to_string(),rb);
                //let mut databases = self.databases;
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
            Ok(())
        }

        ///Get the database rbatis from the pool
        async fn get_db_rb(&self,db_name:&str) -> Option<Arc<rbatis::RBatis>>{
            self.rb_map.get(db_name)
        }

        async fn get_database_(&self,db_name:&str)->Option<Database>{
            self.databases.get(db_name)
            //let mut dbs = self.databases.lock().unwrap();
            //dbs.get_mut(db_name).cloned()
        }

        //connect to rbatis if it isnt cached
        async fn rbatis_connect(&self,db_name:&str)->Result<Option<Arc<rbatis::RBatis>>,rbdc::Error>{
            let cached_rb = self.get_db_rb(db_name).await;
            if cached_rb.is_some(){
                return Ok(cached_rb);
            }
            
            let url = "postgresql://mzeteny:zetou123@localhost:5432/".to_string()+db_name;
            let _ = self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => Some(rb),
                None => return Err(rbdc::Error::from("database not found")),
            };
            Ok(rb)
        }
    }


    impl DatabaseRepository for PostgresRepository{

        ///Get all databases
        async fn get_databases(&self)-> Result<Value,rbdc::Error> {
            log::info!("PgRepository: Get databases");
            let rb = self.rbatis_connect("postgres").await?.unwrap();
            let _sql = "SELECT datname 
            FROM pg_database 
            WHERE datistemplate=false;";
            let result = rb.query(_sql,vec![]).await?;

            ///iterate through databases and insert into the pool and the database map(db.1 = database name)
            if let Some(databases) = result.as_array(){
                for db_val in databases{
                    for db in db_val{
                        if !rb_map.contains_key(db.1.as_str().unwrap()){
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
            }

            Ok(result)
        }

        ///Get all tables in the database
        async fn get_tables(&self, db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();
            let _sql = "SELECT table_name
            FROM information_schema.tables
            WHERE table_schema = 'public';";
            let result = rb.query(_sql,vec![]).await?;
            if let Some(tables) = result.as_array(){
               // let mut db_struct = self.databases.lock().unwrap();
                if let Some(mut node) = self.databases.get(db_name){
                    println!("TABLES: {:?}", tables);
                    let table_map = node.tables.get_or_insert_with(HashMap::new);
                    
                    for table in tables{
                        if let Value::Map(tablemap) = table{
                            let table_name = tablemap.0.get(&Value::String("table_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                        
                            let tb_node = Table{
                                name : table_name.to_string(), 
                                columns : Some(DashMap::new()),
                                constraints: Some(DashMap::new()),
                                indexes: Some(DashMap::new()),
                                triggers: Some(DashMap::new()),
                                rules: Some(DashMap::new()),
                                rls_policies: Some(DashMap::new()),
                            };
                            table_map.insert(table_name.to_string(), tb_node);
                        }
                    }
                    println!("TABLE VALUES COLLECTED: {:?}",db_struct.get(db_name).unwrap().tables.clone().unwrap().into_values().collect::<Vec<Table>>());
                }
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
            
            if let Some(cols) = result.as_array(){
                if let Some(node) = self.databases.get(db_name){
                    let mut table_map: Option<HashMap<String, Table>> = node.tables;
                    let columns_map = table_map.as_mut().unwrap().get_mut(table_name).unwrap().columns.get_or_insert_with(HashMap::new);
                    for col in cols{
                        if let Value::Map(colmap) = col{
                            let col_name = colmap.0.get(&Value::String("column_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let data_type = colmap.0.get(&Value::String("data_type".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let is_nullable = colmap.0.get(&Value::String("is_nullable".to_string())).and_then(|v| v.as_bool()).unwrap_or_default();
                            let column_default = colmap.0.get(&Value::String("column_default".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            
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
            }
            Ok(result)
        }

        async fn get_views(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            
            let _sql = "SELECT table_name
            FROM information_schema.views
            WHERE table_schema = 'public';";
            let result = rb.query(_sql, vec![]).await.unwrap();
            
            if let Some(views) = result.as_array(){
               // let mut db_struct = self.databases.lock().unwrap();
                if let Some(node) = self.databases.get(db_name){
                    let view_map = node.views.get_or_insert_with(HashMap::new);
                    for view in views{
                        if let Value::Map(viewmap) = view{
                            let view_name = viewmap.0.get(&Value::String("table_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let view_node = View{
                                name : view_name.to_string(),
                                definition : "View ".to_string(),
                            };
                            view_map.insert(view_name.to_string(), view_node);
                        }
                    }
                }
            }

            Ok(result)
        }

        async fn get_stored_procedures(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT routine_name
            FROM information_schema.routines
            WHERE routine_type = 'PROCEDURE' AND specific_schema = 'public';";
            let result = rb.query(_sql, vec![]).await.unwrap();

            if let Some(stored_procedures) = result.as_array(){
                //let mut db_struct = self.databases.lock().unwrap();
                if let Some(node) = self.databases.get(db_name){
                    let stored_procedure_map = node.procedures.get_or_insert_with(HashMap::new);
                    for stored_procedure in stored_procedures{
                        if let Value::Map(stored_proceduremap) = stored_procedure{
                            let stored_procedure_name = stored_proceduremap.0.get(&Value::String("routine_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let stored_procedure_node = Procedure{
                                name : stored_procedure_name.to_string(),
                                definition : "Stored Procedure ".to_string(),
                            };
                            stored_procedure_map.insert(stored_procedure_name.to_string(), stored_procedure_node);
                        }
                    }
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

            if let Some(functions) = result.as_array(){
                //let mut db_struct = self.databases.lock().unwrap();
                if let Some(node) = self.databases.get(db_name){
                    let function_map = node.functions.get_or_insert_with(HashMap::new);
                    for function in functions{
                        if let Value::Map(functionmap) = function{
                            let function_name = functionmap.0.get(&Value::String("function_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let function_node = Function{
                                name : function_name.to_string(),
                                definition : "Function ".to_string(),
                            };
                        }
                    }
                }
            }
            Ok(result)
        }

        async fn get_trigger_functions(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql =  "SELECT tgname 
            FROM pg_trigger;";
            let result = rb.query(_sql, vec![]).await.unwrap();
            if let Some(triggers) = result.as_array(){
                //let mut db_struct = self.databases.lock().unwrap();
                if let Some(node) = self.databases.get(db_name){
                    let trigger_map = node.triggers.get_or_insert_with(HashMap::new);
                    for trigger in triggers{
                        if let Value::Map(triggermap) = trigger{
                            let trigger_name = triggermap.0.get(&Value::String("tgname".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let trigger_node = trigger::Trigger{
                                name : trigger_name.to_string(),
                            };
                            trigger_map.insert(trigger_name.to_string(), trigger_node);
                        }
                    }
                }
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
            if let Some(aggregates) = result.as_array(){
               // let mut db_struct = self.databases.lock().unwrap();
                if let Some(node) = self.databases.get(db_name){
                    let aggregate_map = node.aggregates.get_or_insert_with(HashMap::new);
                    for aggregate in aggregates{
                        if let Value::Map(aggregatemap) = aggregate{
                            let aggregate_name = aggregatemap.0.get(&Value::String("proname".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let aggregate_node = Aggregate{
                                name : aggregate_name.to_string(),
                            };
                            aggregate_map.insert(aggregate_name.to_string(), aggregate_node);
                        }
                    }
                }
            }
            Ok(result)
        }

        async fn get_materalized_views(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT matviewname,definition
            FROM pg_matviews;";
            let result = rb.query(_sql, vec![]).await.unwrap();

            if let Some(matview) = result.as_array(){
               // let mut db_struct = self.databases.lock().unwrap();
                if let Some(node) = self.databases.get(db_name){
                    let matview_map = node.materalized_views.get_or_insert_with(HashMap::new);
                    for matview in matview{
                        if let Value::Map(matviewmap) = matview{
                            let matview_name = matviewmap.0.get(&Value::String("matviewname".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let matview_def = matviewmap.0.get(&Value::String("definition".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let matview_node = MateralizedView{
                                name : matview_name.to_string(),
                                definition : matview_def.to_string(),
                            };
                            matview_map.insert(matview_name.to_string(), matview_node);
                        }
                    }
                }
            }
            Ok(result)
        }

        async fn get_types(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            /*
            SELECT t.typname AS type_name, 
                CASE
                    WHEN t.typtype = 'b' THEN 'Base Type'
                    WHEN t.typtype = 'c' THEN 'Composite Type'
                    WHEN t.typtype = 'd' THEN 'Domain'
                    WHEN t.typtype = 'e' THEN 'Enum'
                    WHEN t.typtype = 'r' THEN 'Range'
                    ELSE 'Other'
                END AS type_category
            FROM pg_type t
            JOIN pg_namespace n ON n.oid = t.typnamespace
            WHERE n.nspname NOT IN ('pg_catalog', 'information_schema')
            ORDER BY type_category, type_name;
            
            */
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
            let rb = match self.get_db_rb(db_name).await{
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

            if let Some(catalogs) = result.as_array(){
               // let mut db_struct = self.databases.lock().unwrap();
                if let Some(node) = self.databases.get(db_name){
                    let catalog_map = node.catalogs.get_or_insert_with(HashMap::new);
                    for catalog in catalogs{
                        if let Value::Map(catalogmap) = catalog{
                            let catalog_name = catalogmap.0.get(&Value::String("catalog_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            
                            let catalog_node = Catalog{
                                name : catalog_name.to_string(),
                            };
                            
                            catalog_map.insert(catalog_name.to_string(), catalog_node);
                        }
                    }
                }
            }
            Ok(result)
        }

        async fn get_foreign_data_wrappers(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT fdwname 
            FROM pg_foreign_data_wrapper;";
            let result = rb.query(_sql, vec![]).await.unwrap();

            if let Some(foreign_data_wrappers) = result.as_array(){
                //let mut db_struct = self.databases.lock().unwrap();
                if let Some(node) = self.databases.get(db_name){
                    let foreign_data_wrapper_map = node.foreign_data_wrappers.get_or_insert_with(HashMap::new);
                    for foreign_data_wrapper in foreign_data_wrappers{
                        if let Value::Map(fdw_map) = foreign_data_wrapper{
                            let foreign_data_wrapper_name = fdw_map.0.get(&Value::String("fdwname".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            
                            let foreign_data_wrapper_node = crate::metadata::foreign_data_wrapper::ForeignDataWrapper{
                                name : foreign_data_wrapper_name.to_string(),
                            };
                            
                            foreign_data_wrapper_map.insert(foreign_data_wrapper_name.to_string(), foreign_data_wrapper_node);
                        }
                    }
                }
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
            
            if let Some(indexes) = result.as_array(){
                //let mut db_struct = self.databases.lock().unwrap();
                if let Some(node) = self.databases.get(db_name){
                    let mut table_map: Option<HashMap<String, Table>> = node.tables;
                    let index_map = table_map.as_mut().unwrap().get(table_name).unwrap().indexes.get_or_insert_with(HashMap::new());
                    for index in indexes{
                        if let Value::Map(indexmap) = index{
                            let index_name = indexmap.0.get(&Value::String("indexname".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let index_definition = indexmap.0.get(&Value::String("indexdef".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            
                            let index_node = crate::metadata::index::Index{
                                name : String::from(index_name),
                                definition: Some(String::from(index_definition)),     
                            };
                            index_map.insert(index_name.to_string(), index_node);
                        }
                    }
                }
            }

            Ok(result)
        }

        async fn get_constraints(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT constraint_name, constraint_type
            FROM information_schema.table_constraints 
            WHERE table_schema = 'public';";
            let result = rb.query(_sql, vec![]).await.unwrap();
            if let Some(constraints) = result.as_array(){
                //let mut db_struct = self.databases.lock().unwrap();
                if let Some(node) = self.databases.get(db_name){
                    let mut table_map: Option<HashMap<String, Table>> = node.tables;
                    let constraint_map = table_map.as_mut().unwrap().get(table_name).unwrap().constraints.get_or_insert_with(HashMap::new());
                    for constraint in constraints{
                        if let Value::Map(colmap) = constraint{
                            let constraint_name = colmap.0.get(&Value::String("constraint_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let constraint_type = colmap.0.get(&Value::String("constraint_type".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            
                            let constraint_node = crate::metadata::constraint::Constraint{
                                name: String::from(constraint_name),
                                c_type: String::from(constraint_type)
                            };
                            constraint_map.insert(constraint_name.to_string(), constraint_node);
                        }
                    }
                }
            }
            Ok(result)

        }

        async fn get_sequences(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let rb = self.rbatis_connect(db_name).await?.unwrap();

            let _sql = "SELECT sequence_name 
            FROM information_schema.sequences 
            WHERE sequence_schema = 'public';";
            let result = rb.query(_sql, vec![]).await.unwrap();

            if let Some(sequences) = result.as_array(){
                //let mut db_struct = self.databases.lock().unwrap();
                if let Some(node) = self.databases.get(db_name){
                    let sequence_map = node.sequences.get_or_insert_with(HashMap::new);
                    for seq in sequences{
                        if let Value::Map(sequencemap) = seq{
                            let sequence_name = sequencemap.0.get(&Value::String("sequence_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let sequence_node = Sequence{
                                name : sequence_name.to_string(),
                            };
                            sequence_map.insert(sequence_name.to_string(), sequence_node);
                        }
                    }
                }
                    
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
            let rb = match self.get_db_rb("postgres").await{
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

            if let Some(policies) = result.as_array(){
               // let mut db_struct = self.databases.lock().unwrap();
                if let Some(node) = self.databases.get(db_name){
                    let mut table_map: Option<HashMap<String, Table>> = node.tables;
                    let policy_map = table_map.as_mut().unwrap().get(table_name).unwrap().rls_policies.get_or_insert_with(HashMap::new());
                    for policy in policies{
                        if let Value::Map(polmap) = policy{
                            let policy_name = polmap.0.get(&Value::String("policy_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let policy_node = RlsPolicy{
                                name : policy_name.to_string(),
                                command : polmap.0.get(&Value::String("command".to_string())).and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                            };
                            policy_map.insert(policy_name.to_string(), policy_node);
                        }
                    }
                }
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
            
            if let Some(rules) = result.as_array(){
                //let mut db_struct = self.databases.lock().unwrap();
                if let Some(mut node) = self.databases.get(db_name){
                    let table_map = node.tables.as_mut().unwrap().get(table_name);
                    let rule_map = table_map.unwrap().rules.get_or_insert_with(HashMap::new);//table_map.unwrap().get(table_name).unwrap().rules.get_or_insert_with(HashMap::new);//table_map.as_mut().unwrap().get(table_name).unwrap().rules.get_or_insert_with(HashMap::new());
                    for rule in rules{
                        if let Value::Map(rulemap) = rule{
                            let rule_name = rulemap.0.get(&Value::String("rule_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let rule_node = Rule{
                                name : rule_name.to_string(),
                                definition : rulemap.0.get(&Value::String("rule_definition".to_string())).and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                            };
                            rule_map.insert(rule_name.to_string(), rule_node);
                        }
                    }
                }   
            }

            Ok(result)

        }
        
    }

