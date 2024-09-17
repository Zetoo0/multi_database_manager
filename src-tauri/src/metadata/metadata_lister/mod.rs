use crate::table::Table;
use crate::user::User;

pub trait MetadataLister{
    pub fn list_databases();
    pub fn list_columns(table:Table);
    pub fn list_views();
    pub fn list_stored_procedures();
    pub fn list_functions();
    pub fn list_trigger_functions();
    pub fn list_event_triggers();
    pub fn list_aggregates();
    pub fn list_materalized_views();
    pub fn list_types();
    pub fn list_languages();
    pub fn list_catalogs();
    pub fn list_foreign_data_wrappers();
    pub fn list_schemas();
    pub fn list_indexes();
    pub fn list_constraints();
    pub fn list_sequences();
    pub fn list_roles_and_users();
    pub fn list_table_statistics();
    pub fn list_active_sessions();
    pub fn list_locks();
    pub fn list_partitions(table:Table);
    pub fn list_user_privileges(user:User);
    pub fn list_database_settings();
    pub fn list_foreign_key_relationships();
    pub fn list_triggers_associated_with_table(table:Table);
    pub fn list_default_columns_value(table:Table);
}