use crate::metadata::repository::DatabaseRepository;
use fast_pool::Pool;
use rbatis::{executor::RBatisRef, DefaultPool};
use rbdc::db::{self, ConnectOptions};
use rbdc_pg::connection::PgConnection;
use rbdc_pg::*;
use rbs::to_value;
use std::{collections::HashMap, future::Future, sync::Mutex};
use rbdc::Error;
use rbs::Value;
use serde::{Serialize,Deserialize};
use std::sync::Arc;

#[derive(Debug)]
pub struct PostgresRepository{
    rb_map:Arc<Mutex<HashMap<String,Arc<rbatis::RBatis>>>>,
}

impl PostgresRepository{
    pub fn new() -> Self{
        /*let pg_connectioptions = PgConnectOptions::new();
        pg_connectioptions.host("localhost");
        pg_connectioptions.port(5432);
        pg_connectioptions.username("postgres");
        pg_connectioptions.password("postgres");
        let pg_connection = PgConnection::establish(&pg_connectioptions.clone());*/
        //let rb = rbatis::RBatis::new();
        //let url = "postgresql://mzeteny:zetou123@localhost:5432/postgres";
        //let _ = rb.init(PgDriver {}, url);
        //self.rb.init_pool(PgDriver {}, "postgresql://mzeteny:zetou123@localhost:5432/postgres").unwrap();
        let rb_map = Arc::new(Mutex::new(HashMap::new()));
    
        return PostgresRepository{ rb_map };
    }

    async fn connect(&self,db_name:&str,url:&str) -> Result<(), Box<dyn std::error::Error>> {
        println!("asd???");
        let mut rb_map = self.rb_map.lock().unwrap();
        println!("Connecting to... {}", url);
        if !rb_map.contains_key(db_name){
            println!("{} not in rb_map",db_name);
            let rb = Arc::new(rbatis::RBatis::new());
            let _ = rb.init(PgDriver {}, url);
            rb_map.insert(db_name.to_string(),rb);
        }

        Ok(())
    }

    async fn get_db_rb(&self,db_name:&str) -> Option<Arc<rbatis::RBatis>>{
        let rb_map = self.rb_map.lock().unwrap();
        rb_map.get(db_name).cloned()
    }
}


impl DatabaseRepository for PostgresRepository{

    async fn get_databases(&self)-> Result<Value,rbdc::Error> {
        let url = "postgresql://mzeteny:zetou123@localhost:5432/postgres";
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
        let url = "postgresql://mzeteny:zetou123@localhost:5432/".to_string()+db_name;
        let _ = self.connect(db_name, url.as_str()).await;
        let rb = match self.get_db_rb(db_name).await{
            Some(rb) => rb,
            None => return Err(rbdc::Error::from("database not found")),
        };        //self.rb.link(PgDriver {}, url.as_str());
        let _sql = "SELECT table_name
        FROM information_schema.tables
        WHERE table_schema = 'public';";
        let result = rb.query(_sql,vec![]).await.unwrap();
        Ok(result)
        
    }

    async fn get_columns(&self,table_name:&str)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT column_name, data_type
        FROM information_schema.columns
        WHERE table_name = ?;";
       // let tables = self.rb.query(_sql, table_name)
        //Ok(self.rb.query(_sql,vec![to_value!(table_name)]).await?)
        Ok(Value::Null)
    }

    async fn get_views(&self)-> Result<Value,rbdc::Error> {
       let _sql = "SELECT table_name
        FROM information_schema.views
        WHERE table_schema = 'public';";
        
        Ok(Value::Null)

    }

    async fn get_stored_procedures(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT routine_name
        FROM information_schema.routines
        WHERE routine_type = 'PROCEDURE' AND specific_schema = 'public';";
        
        Ok(Value::Null)

    }

    async fn get_functions(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT routine_name, routine_type
            FROM information_schema.routines
            WHERE routine_type = 'FUNCTION';";
        Ok(Value::Null)

    }

    async fn get_trigger_functions(&self)-> Result<Value,rbdc::Error> {
       let _sql =  "SELECT tgname 
        FROM pg_trigger;";
        Ok(Value::Null)

    }

    async fn get_event_triggers(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT evtname 
        FROM pg_event_trigger;";
        Ok(Value::Null)

    }

    async fn get_aggregates(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT proname 
        FROM pg_proc 
        WHERE proisagg = true;";
        Ok(Value::Null)

    }

    async fn get_materalized_views(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT matviewname 
        FROM pg_matviews;";
        Ok(Value::Null)

    }

    async fn get_types(&self)-> Result<Value,rbdc::Error> {
       let _sql = "SELECT typname 
        FROM pg_type 
        WHERE typtype = 'b';";
        Ok(Value::Null)

    }

    async fn get_languages(&self)-> Result<Value,rbdc::Error> {
        let _sql ="SELECT lanname 
        FROM pg_language;";
        Ok(Value::Null)

    }

    async fn get_catalogs(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT datname 
        FROM pg_database;";
        Ok(Value::Null)

    }

    async fn get_foreign_data_wrappers(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT fdwname 
        FROM pg_foreign_data_wrapper;";
        Ok(Value::Null)

    }

    async fn get_schemas(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT schema_name 
        FROM information_schema.schemata;";
        Ok(Value::Null)
    }

    async fn get_indexes(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT indexname, indexdef, indrelid, indkey
        FROM pg_indexes 
        WHERE schemaname = 'public';";
        Ok(Value::Null)

    }

    async fn get_constraints(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT constraint_name, constraint_type
        FROM information_schema.table_constraints 
        WHERE table_schema = 'public';";
        Ok(Value::Null)

    }

    async fn get_sequences(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT sequence_name 
        FROM information_schema.sequences 
        WHERE sequence_schema = 'public';";
        Ok(Value::Null)

    }

    async fn get_roles_and_users(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT rolname 
        FROM pg_roles;";
        Ok(Value::Null)

    }

    async fn get_table_statistics(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT relname, n_live_tup, n_dead_tup
        FROM pg_stat_user_tables;";
        Ok(Value::Null)

    }

    async fn get_active_sessions(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT pid, usename, application_name, client_addr 
        FROM pg_stat_activity;";
        Ok(Value::Null)

    }

    async fn get_locks(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT * FROM pg_locks;";
        Ok(Value::Null)

    }

    async fn get_partitions(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT * 
        FROM pg_partitions 
        WHERE tablename = 'your_table_name';";
        Ok(Value::Null)

    }

    async fn get_user_privileges(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT grantee, privilege_type, table_name 
        FROM information_schema.role_table_grants 
        WHERE grantee = 'your_user';";
        Ok(Value::Null)


    }

    async fn get_database_settings(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SHOW ALL;";
        Ok(Value::Null)

    }

    async fn get_foreign_key_relationships(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT conname AS constraint_name, conrelid::regclass AS table_name,
       a.attname AS column_name, confrelid::regclass AS foreign_table_name,
       af.attname AS foreign_column_name
        FROM   pg_constraint
        JOIN   pg_attribute a ON a.attnum = ANY(conkey) AND a.attrelid = conrelid
        JOIN   pg_attribute af ON af.attnum = ANY(confkey) AND af.attrelid = confrelid
        WHERE  contype = 'f';";
        Ok(Value::Null)


    }

    async fn get_triggers_associated_with_table(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT tgname
        FROM pg_trigger
        WHERE tgrelid = 'your_table_name'::regclass;";
        Ok(Value::Null)

    }

    async fn get_default_columns_value(&self)-> Result<Value,rbdc::Error> {
        let _sql = "SELECT column_name, column_default
        FROM information_schema.columns
        WHERE table_name = 'your_table_name';";
        Ok(Value::Null)

    }
    
}
