use crate::domain::create_info::create_table_info::{CreateSequenceInfo, CreateTableInfo};
use crate::domain::metadata::column::Column;
use crate::domain::metadata::constraint::Constraint;
use crate::domain::metadata::database::Schema;
use crate::domain::metadata::index::Index;
use crate::domain::metadata::role::Role;
use crate::domain::metadata::trigger::Trigger;
use crate::domain::metadata::view::View;
use crate::domain::{
    datb::{migraton_config::MigrationConfig, query_info::QueryInfo},
    metadata::{
        database_metadata::DatabaseMetadata, function::Function, sequence::Sequence, table::Table,
    },
};
use dashmap::DashMap;
use futures::Future;
use rbs::value::map::ValueMap;
use std::pin::Pin;

pub trait DatabaseService: Send + Sync {
    fn init_database<'a>(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<(), rbdc::error::Error>> + Send + 'a>>;
    fn get_metadatas(
        &self,
        db_name: &str,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DashMap<String, DatabaseMetadata>, rbdc::error::Error>>
                + Send,
        >,
    >;
    fn migrate_to<'a>(
        &self,
        migration_conf: MigrationConfig, /* ,table_name:&'a str,columns:&'a Vec<String>*/
    ) -> Pin<Box<dyn Future<Output = Result<(), rbdc::error::Error>> + Send + 'a>>;
    fn _query<'a>(
        &self,
        query_info: QueryInfo,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ValueMap>, rbdc::error::Error>> + Send + 'a>>;
    fn create_database<'a>(&self, _db_name: &'a str, path: &'a str) -> Pin<Box<dyn Future<Output = Result<(), rbdc::error::Error>> + Send + 'a>>;
    fn create_table<'a>(
        &self,
        table_info: CreateTableInfo,
        db_name: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<Table, rbdc::error::Error>> + Send + 'a>>;
    fn create_sequence<'a>(
        &self,
        seq: CreateSequenceInfo,
        db_name: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<Sequence, rbdc::error::Error>> + Send + 'a>>;
    fn create_function<'a>(
        &self,
        function: Function,
    ) -> Pin<Box<dyn Future<Output = Result<Function, rbdc::error::Error>> + Send + 'a>>;
    fn create_index<'a>(
        &self,
        index:Index,
        db_name: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<Index, rbdc::error::Error>> + Send + 'a>>;
    fn create_view<'a>(
        &self,
        view:View,
        db_name: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<View, rbdc::error::Error>> + Send + 'a>>;
    fn create_role<'a>(
        &self,
        role:Role,
        db_name: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<Role, rbdc::error::Error>> + Send + 'a>>;
    fn create_schema<'a>(
        &self,
        name:&'a str,
        db_name: &'a str,
        user_name:Option<String>
    ) -> Pin<Box<dyn Future<Output = Result<Schema, rbdc::error::Error>> + Send + 'a>>;
    fn create_constraint<'a>(
        &self,
        constraint: Constraint,
        table_name: &'a str,
        schema_name: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<Constraint, rbdc::error::Error>> + Send + 'a>>;
    fn create_trigger<'a>(
        &self,
        name:&'a str,
        when:&'a str,
        type_:&'a str,
        table_name:&'a str,
        function_name:&'a str,
        db_name: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<Trigger, rbdc::error::Error>> + Send + 'a>>;

    fn add_column<'a>(
        &self,
        table_name: String,
        column_info: Column,
        database_name:String
    ) -> Pin<Box<dyn Future<Output = Result<Column, ()>> + Send + 'a>>;
    fn edit_table_column<'a>(
        &self,
        table_name: String,
        db_name: String,
        new_cols: Column,
        old_cols: Column,
    ) -> Pin<Box<dyn Future<Output = Result<Column, rbdc::error::Error>> + Send + 'a>>;
    fn get_table<'a>(
        &self,
        table_name: String,
        database_name: String
    ) -> Pin<Box<dyn Future<Output = Result<Table, ()>> + Send + 'a>>;
    fn edit_sequence<'a>(
        &self,
        db_name: String,
        old_sequence: Sequence,
        new_sequence: Sequence,
    ) -> Pin<Box<dyn Future<Output = Result<Sequence, rbdc::error::Error>> + Send + 'a>>;
    fn edit_function<'a>(
        &self,
        db_name: String,
        old_function: Function,
        new_function: Function,
    ) -> Pin<Box<dyn Future<Output = Result<Function, rbdc::error::Error>> + Send + 'a>>;
    fn edit_constraint<'a>(
        &self,
        db_name: String,
        table_name:String,
        old_constraint: Constraint,
        new_constraint: Constraint,
    ) -> Pin<Box<dyn Future<Output = Result<Constraint, rbdc::error::Error>> + Send + 'a>>;
    fn edit_index<'a>(
        &self,
        db_name: String,
        table_name: String,    
        old_index: Index,
        new_index: Index,
    ) -> Pin<Box<dyn Future<Output = Result<Index, rbdc::error::Error>> + Send + 'a>>;
    fn delete_table_column<'a>(
        &self,
        column_name: String,
        table_name: String,
        db_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), rbdc::Error>> + Send + 'a>>;
    fn delete_table<'a>(
        &self,
        table_name: String,
        db_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>>;
    fn delete_sequence<'a>(
        &self,
        sequence_name: String,
        db_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>>;
    fn delete_function<'a>(
        &self,
        function_name: String,
        db_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>>;
    fn base_delete<'a>(
        &self,
        delete_to_name: String,
        object_name: String,
        db_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), rbdc::Error>> + Send + 'a>>;

}
