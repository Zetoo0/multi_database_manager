    use crate::metadata::{self, repository::DatabaseRepository, table::Table};
    use fast_pool::Pool;
    use rbatis::{executor::RBatisRef, DefaultPool};
    use rbdc::db::{self, ConnectOptions};
    use rbdc_mssql::tiberius::Column;
    use rbdc_pg::connection::PgConnection;
    use rbdc_pg::*;
    use rbs::to_value;
    use std::{collections::HashMap, future::Future, result, sync::Mutex};
    use rbdc::Error;
    use rbs::Value;
    use serde::{Serialize,Deserialize};
    use std::sync::Arc;
    use log::{error, info, warn};
    use fast_log::init;
    use crate::metadata::database::Database;

    #[derive(Debug)]
    pub struct PostgresRepository{
        rb_map:Arc<Mutex<HashMap<String,Arc<rbatis::RBatis>>>>,
        base_url:String,
        pub databases:Arc<Mutex<HashMap<String,Database>>>,
    }

    impl PostgresRepository{
        pub fn new() -> Self{
            let rb_map = Arc::new(Mutex::new(HashMap::new()));
            let databases = Arc::new(Mutex::new(HashMap::new()));
            let base_url = String::from("postgresql://postgres:@localhost:5432/postgres");
            return PostgresRepository{ rb_map, base_url, databases};
        }

        ///Add the database to the pool if not exists
        ///It's create a new rbatis, initialize it add add to the pool
        async fn connect(&self,db_name:&str,url:&str) -> Result<(), Box<dyn std::error::Error>> {
            let mut rb_map = self.rb_map.lock().unwrap();
            if !rb_map.contains_key(db_name){
                log::info!("new pool adding... database: {:?}",db_name);
                println!("new pool adding... database: {:?}",db_name);
                let rb = Arc::new(rbatis::RBatis::new());
                let _ = rb.init(PgDriver {}, url);
                rb_map.insert(db_name.to_string(),rb);

                let mut databases = self.databases.lock().unwrap();
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
                };
                databases.insert(db_name.to_string(), db_node);
            }

            Ok(())
        }

        ///Get the database rbatis from the pool
        async fn get_db_rb(&self,db_name:&str) -> Option<Arc<rbatis::RBatis>>{
            let rb_map = self.rb_map.lock().unwrap();
            rb_map.get(db_name).cloned()
        }

        async fn get_database_(&self,db_name:&str)->Option<Database>{
            let mut dbs = self.databases.lock().unwrap();
            dbs.get_mut(db_name).cloned()
        }
    }


    impl DatabaseRepository for PostgresRepository{

        async fn get_databases(&self)-> Result<Value,rbdc::Error> {
            log::info!("PgRepository: Get databases");
            let url = "postgresql://postgres:@localhost:5432/postgres";
            let _ = self.connect("postgres", url).await;
            let rb = match self.get_db_rb("postgres").await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql = "SELECT datname 
            FROM pg_database 
            WHERE datistemplate=false;";
            let result = rb.query(_sql,vec![]).await.unwrap();
            
            Ok(result)
        }

        async fn get_tables(&self, db_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            let _ = self.connect(db_name, url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };        //self.rb.link(PgDriver {}, url.as_str());
            let _sql = "SELECT table_name
            FROM information_schema.tables
            WHERE table_schema = 'public';";
            let result = rb.query(_sql,vec![]).await.unwrap();            
            if let Some(tables) = result.as_array(){
                let mut db_struct = self.databases.lock().unwrap();
                if let Some(node) = db_struct.get_mut(db_name){
                    println!("TABLES: {:?}", tables);
                    let table_map = node.tables.get_or_insert_with(HashMap::new);
                    for table in tables{
                        for table_name in table{
                            if let Some(table_namae) = table_name.1.as_str(){      
                                let tb_node = Table{
                                    name : table_namae.to_string(), 
                                    columns : Some(HashMap::new()),
                                };
                                table_map.insert(table_namae.to_string(), tb_node);
                                //table_map.as_mut().expect("asd").insert(table_namae.to_string(), tb_node);
                            }
                            //table_map.insert("product".to_string(), Table{name:"product".to_string(),columns:Some(HashMap::new())});
                        }
                        
                    }   
                }
            }
            Ok(result)
        }

        async fn get_columns(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            let _ = self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };       
            let _sql = "SELECT column_name
            FROM information_schema.columns
            WHERE table_name = ?;";
            //Ok(self.rb.query(_sql,vec![to_value!(table_name)]).await?)
            let result = rb.query(_sql, vec![Value::String(table_name.to_string())]).await.unwrap();
            if let Some(cols) = result.as_array(){
                let mut db_struct = self.databases.lock().unwrap();
                if let Some(node) = db_struct.get_mut(db_name){
                    let _tb_struct = node.tables.as_mut().expect("Expected some table").get_mut(table_name);
                    let columns_map = _tb_struct.expect("msg").columns.get_or_insert_with(HashMap::new);
                    for col in cols{
                        for col_name in col{
                            println!("KOLOS: {:?}", col_name.1);    
                            let _col_node = crate::metadata::column::Column{
                                name : col_name.1.to_string(),
                                data_type: Some(String::from("value")),     
                                is_nullable: Some(true),
                                default_value: Some(String::from("Anzas")),
                            };
                            columns_map.insert(col_name.1.to_string(), _col_node);
                        }
                    }

                }
            }
            
            Ok(result)
        }

        async fn get_views(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
        let _sql = "SELECT table_name
            FROM information_schema.views
            WHERE table_schema = 'public';";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)
        }

        async fn get_stored_procedures(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql = "SELECT routine_name
            FROM information_schema.routines
            WHERE routine_type = 'PROCEDURE' AND specific_schema = 'public';";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)
        }

        async fn get_functions(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
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
            Ok(result)
        }

        async fn get_trigger_functions(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql =  "SELECT tgname 
            FROM pg_trigger;";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)
        }

        async fn get_event_triggers(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql = "SELECT evtname 
            FROM pg_event_trigger;";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)

        }

        async fn get_aggregates(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql = "SELECT proname 
            FROM pg_proc 
            WHERE proisagg = true;";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)
        }

        async fn get_materalized_views(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql = "SELECT matviewname 
            FROM pg_matviews;";
            let result = rb.query(_sql, vec![]).await.unwrap();
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
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
        let _sql = "SELECT typname 
            FROM pg_type 
            WHERE typtype = 'b';";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)
        }

        async fn get_languages(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
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
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql = "SELECT nspname AS catalog_name
                            FROM pg_namespace
                            WHERE nspname IN ('pg_catalog', 'information_schema')
                            ORDER BY nspname;";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)
        }

        async fn get_foreign_data_wrappers(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql = "SELECT fdwname 
            FROM pg_foreign_data_wrapper;";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)

        }

        async fn get_schemas(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql = "SELECT schema_name 
            FROM information_schema.schemata
            WHERE schema_name='public';";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)
        }

        async fn get_indexes(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql = "SELECT indexname, indexdef
            FROM pg_indexes 
            WHERE schemaname = 'public';";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)

        }

        async fn get_constraints(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql = "SELECT constraint_name, constraint_type
            FROM information_schema.table_constraints 
            WHERE table_schema = 'public';";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)

        }

        async fn get_sequences(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql = "SELECT sequence_name 
            FROM information_schema.sequences 
            WHERE sequence_schema = 'public';";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)

        }

        async fn get_roles_and_users(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql = "SELECT rolname FROM pg_roles;";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)

        }

        async fn get_table_statistics(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
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
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql = "SELECT * FROM pg_locks;";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)

        }

        async fn get_partitions(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
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
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql = "SELECT grantee, privilege_type, table_name 
            FROM information_schema.role_table_grants 
            WHERE grantee = ?;";
            let result = rb.query(_sql, vec![Value::String(user_name.to_string())]).await.unwrap();
            Ok(result)
        }

        async fn get_database_settings(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql = "SHOW ALL;";
            let result = rb.query(_sql, vec![]).await.unwrap();
            Ok(result)
        }

        async fn get_foreign_key_relationships(&self,db_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
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
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql = "SELECT tgname
            FROM pg_trigger
            WHERE tgrelid = ?::regclass;";
            let result = rb.query(_sql, vec![Value::String(table_name.to_string())]).await.unwrap();
            Ok(result)
        }

        async fn get_default_columns_value(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
            let url = "postgresql://postgres:@localhost:5432/".to_string()+db_name;
            self.connect(db_name,url.as_str()).await;
            let rb = match self.get_db_rb(db_name).await{
                Some(rb) => rb,
                None => return Err(rbdc::Error::from("database not found")),
            };
            let _sql = "SELECT column_name, column_default
            FROM information_schema.columns
            WHERE table_name = ?;";
            let result = rb.query(_sql, vec![Value::String(table_name.to_string())]).await.unwrap();
            Ok(result)

        }
        
    }
