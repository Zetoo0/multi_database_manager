pub mod PostgresRepository;

use std::future::Future;
use rbdc::{db, Error};
use rbs::Value;

pub trait DatabaseRepository{
     async fn get_databases(&self)-> Result<Value,rbdc::Error>;
     async fn get_tables(&self, db_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_columns(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_views(&self,db_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_stored_procedures(&self,db_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_functions(&self,db_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_trigger_functions(&self,db_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_event_triggers(&self,db_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_aggregates(&self,db_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_materalized_views(&self,db_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_types(&self,db_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_languages(&self,db_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_catalogs(&self,db_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_foreign_data_wrappers(&self,db_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_schemas(&self,db_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_indexes(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_constraints(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_sequences(&self,db_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_roles_and_users(&self,db_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_table_statistics(&self,db_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_active_sessions(&self)-> Result<Value,rbdc::Error>;
     async fn get_locks(&self,db_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_partitions(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_user_privileges(&self,db_name:&str,user_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_database_settings(&self,db_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_foreign_key_relationships(&self,db_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_triggers_associated_with_table(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_default_columns_value(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_rls_policies(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_rules(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error>;
}