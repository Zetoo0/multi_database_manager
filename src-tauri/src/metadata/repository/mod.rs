pub mod PostgresRepository;

use std::future::Future;
use rbdc::Error;
use rbs::Value;

pub trait DatabaseRepository{

     async fn get_databases(&self)-> Result<Value,rbdc::Error>;
     async fn get_tables(&self, db_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_columns(&self,table_name:&str)-> Result<Value,rbdc::Error>;
     async fn get_views(&self)-> Result<Value,rbdc::Error>;
     async fn get_stored_procedures(&self)-> Result<Value,rbdc::Error>;
     async fn get_functions(&self)-> Result<Value,rbdc::Error>;
     async fn get_trigger_functions(&self)-> Result<Value,rbdc::Error>;
     async fn get_event_triggers(&self)-> Result<Value,rbdc::Error>;
     async fn get_aggregates(&self)-> Result<Value,rbdc::Error>;
     async fn get_materalized_views(&self)-> Result<Value,rbdc::Error>;
     async fn get_types(&self)-> Result<Value,rbdc::Error>;
     async fn get_languages(&self)-> Result<Value,rbdc::Error>;
     async fn get_catalogs(&self)-> Result<Value,rbdc::Error>;
     async fn get_foreign_data_wrappers(&self)-> Result<Value,rbdc::Error>;
     async fn get_schemas(&self)-> Result<Value,rbdc::Error>;
     async fn get_indexes(&self)-> Result<Value,rbdc::Error>;
     async fn get_constraints(&self)-> Result<Value,rbdc::Error>;
     async fn get_sequences(&self)-> Result<Value,rbdc::Error>;
     async fn get_roles_and_users(&self)-> Result<Value,rbdc::Error>;
     async fn get_table_statistics(&self)-> Result<Value,rbdc::Error>;
     async fn get_active_sessions(&self)-> Result<Value,rbdc::Error>;
     async fn get_locks(&self)-> Result<Value,rbdc::Error>;
     async fn get_partitions(&self)-> Result<Value,rbdc::Error>;
     async fn get_user_privileges(&self)-> Result<Value,rbdc::Error>;
     async fn get_database_settings(&self)-> Result<Value,rbdc::Error>;
     async fn get_foreign_key_relationships(&self)-> Result<Value,rbdc::Error>;
     async fn get_triggers_associated_with_table(&self)-> Result<Value,rbdc::Error>;
     async fn get_default_columns_value(&self)-> Result<Value,rbdc::Error>;
}