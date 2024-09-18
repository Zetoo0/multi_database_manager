pub mod PostgresRepository;

use std::future::Future;
use rbdc::Error;
use rbs::Value;

pub trait DatabaseRepository{
    type Connection;
    type RBatis;

     fn get_databases(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_tables(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_columns(&self,table_name:&str)-> impl Future<Output = Result<Value, Error>>;
     fn get_views(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_stored_procedures(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_functions(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_trigger_functions(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_event_triggers(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_aggregates(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_materalized_views(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_types(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_languages(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_catalogs(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_foreign_data_wrappers(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_schemas(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_indexes(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_constraints(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_sequences(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_roles_and_users(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_table_statistics(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_active_sessions(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_locks(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_partitions(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_user_privileges(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_database_settings(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_foreign_key_relationships(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_triggers_associated_with_table(&self)-> impl Future<Output = Result<Value, Error>>;
     fn get_default_columns_value(&self)-> impl Future<Output = Result<Value, Error>>;
}