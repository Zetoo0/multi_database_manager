use crate::domain::datb::database_type::DatabaseType;

pub trait GenerateDDL: Send + Sync {
    fn to_create_function(&self, target_db_type: DatabaseType) -> Result<String, String>;
}
