use crate::metadata::repository::DatabaseRepository;
use options::PgConnectOptions;
use rbatis::executor::RBatisRef;
use rbdc::db::ConnectOptions;
use rbdc_pg::connection::PgConnection;
use rbdc_pg::*;
use rbs::to_value;
use std::future::Future;
use rbdc::Error;
use rbs::Value;
use serde::{Serialize,Deserialize};
use rbdc::pool::Pool;



pub struct PostgresRepository<'a>{
    rb:&'a rbatis::RBatis,

}

impl PostgresRepository<'_>{
    pub fn new(rb : &rbatis::RBatis) -> Self{
        /*let pg_connectioptions = PgConnectOptions::new();
        pg_connectioptions.host("localhost");
        pg_connectioptions.port(5432);
        pg_connectioptions.username("postgres");
        pg_connectioptions.password("postgres");
        let pg_connection = PgConnection::establish(&pg_connectioptions.clone());*/
        //let rb = rbatis::RBatis::new();
        let rba = rb.rb_ref();
        let url = "postgres://postgres:postgres@localhost:5432/postgres";
        rba.link(PgDriver {}, url);
        return PostgresRepository{rb:rb.rb_ref()};
    }
}

impl DatabaseRepository for PostgresRepository<'_>{
    type Connection = rbdc_pg::connection::PgConnection;
    type RBatis = rbatis::RBatis;

    fn get_databases(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT datname 
        FROM pg_database 
        WHERE datistemplate=false;";
        return self.rb.query(&_sql,vec![]);
    }

    fn get_tables(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT table_name
        FROM information_schema.tables
        WHERE table_schema = 'public' AND table_type = 'BASE TABLE';";
        return self.rb.query(&_sql,vec![]);
    }

    fn get_columns(&self,table_name:&str)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT column_name, data_type
        FROM information_schema.columns
        WHERE table_name = ?;";
        return self.rb.query(&_sql,vec![to_value!(table_name)]);
    }

    fn get_views(&self)-> impl Future<Output = Result<Value, Error>> {
       let _sql = "SELECT table_name
        FROM information_schema.views
        WHERE table_schema = 'public';";
        
        return self.rb.query(&_sql,vec![]);
    }

    fn get_stored_procedures(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT routine_name
        FROM information_schema.routines
        WHERE routine_type = 'PROCEDURE' AND specific_schema = 'public';";
        
        return self.rb.query(&_sql, vec![]);
    }

    fn get_functions(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT routine_name, routine_type
            FROM information_schema.routines
            WHERE routine_type = 'FUNCTION';";
        return self.rb.query(&_sql, vec![]);
    }

    fn get_trigger_functions(&self)-> impl Future<Output = Result<Value, Error>> {
       let _sql =  "SELECT tgname 
        FROM pg_trigger;";
        return self.rb.query(&_sql, vec![]);
    }

    fn get_event_triggers(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT evtname 
        FROM pg_event_trigger;";
        return self.rb.query(&_sql, vec![]);

    }

    fn get_aggregates(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT proname 
        FROM pg_proc 
        WHERE proisagg = true;";
        return self.rb.query(&_sql, vec![]);

    }

    fn get_materalized_views(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT matviewname 
        FROM pg_matviews;";
        return self.rb.query(&_sql, vec![]);

    }

    fn get_types(&self)-> impl Future<Output = Result<Value, Error>> {
       let _sql = "SELECT typname 
        FROM pg_type 
        WHERE typtype = 'b';";
        return self.rb.query(&_sql, vec![]);

    }

    fn get_languages(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql ="SELECT lanname 
        FROM pg_language;";
        return self.rb.query(&_sql, vec![]);

    }

    fn get_catalogs(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT datname 
        FROM pg_database;";
        return self.rb.query(&_sql, vec![]);

    }

    fn get_foreign_data_wrappers(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT fdwname 
        FROM pg_foreign_data_wrapper;";
        return self.rb.query(&_sql, vec![]);

    }

    fn get_schemas(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT schema_name 
        FROM information_schema.schemata;";
        return self.rb.query(&_sql, vec![]);
    }

    fn get_indexes(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT indexname, indexdef, indrelid, indkey
        FROM pg_indexes 
        WHERE schemaname = 'public';";
        return self.rb.query(&_sql, vec![]);

    }

    fn get_constraints(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT constraint_name, constraint_type
        FROM information_schema.table_constraints 
        WHERE table_schema = 'public';";
        return self.rb.query(&_sql, vec![]);

    }

    fn get_sequences(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT sequence_name 
        FROM information_schema.sequences 
        WHERE sequence_schema = 'public';";
        return self.rb.query(&_sql, vec![]);

    }

    fn get_roles_and_users(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT rolname 
        FROM pg_roles;";
        return self.rb.query(&_sql, vec![]);

    }

    fn get_table_statistics(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT relname, n_live_tup, n_dead_tup
        FROM pg_stat_user_tables;";
        return self.rb.query(&_sql, vec![]);

    }

    fn get_active_sessions(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT pid, usename, application_name, client_addr 
        FROM pg_stat_activity;";
        return self.rb.query(&_sql, vec![]);

    }

    fn get_locks(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT * FROM pg_locks;";
        return self.rb.query(&_sql, vec![]);

    }

    fn get_partitions(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT * 
        FROM pg_partitions 
        WHERE tablename = 'your_table_name';";
        return self.rb.query(&_sql, vec![]);

    }

    fn get_user_privileges(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT grantee, privilege_type, table_name 
        FROM information_schema.role_table_grants 
        WHERE grantee = 'your_user';";
        return self.rb.query(&_sql, vec![]);

    }

    fn get_database_settings(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SHOW ALL;";
        return self.rb.query(&_sql, vec![]);
    }

    fn get_foreign_key_relationships(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT conname AS constraint_name, conrelid::regclass AS table_name,
       a.attname AS column_name, confrelid::regclass AS foreign_table_name,
       af.attname AS foreign_column_name
        FROM   pg_constraint
        JOIN   pg_attribute a ON a.attnum = ANY(conkey) AND a.attrelid = conrelid
        JOIN   pg_attribute af ON af.attnum = ANY(confkey) AND af.attrelid = confrelid
        WHERE  contype = 'f';";
        return self.rb.query(&_sql, vec![]);

    }

    fn get_triggers_associated_with_table(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT tgname
        FROM pg_trigger
        WHERE tgrelid = 'your_table_name'::regclass;";
        return self.rb.query(&_sql, vec![]);
    }

    fn get_default_columns_value(&self)-> impl Future<Output = Result<Value, Error>> {
        let _sql = "SELECT column_name, column_default
        FROM information_schema.columns
        WHERE table_name = 'your_table_name';";
        return self.rb.query(&_sql, vec![]);
    }
    
}
