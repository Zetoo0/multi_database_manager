use serde::{Deserialize, Serialize};

use crate::domain::datb::{
    database_type::DatabaseType,
    dml_generate::{DMLLimit, Obfuscation},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationConfig {
    pub db_name: String,
    pub schema_name: String,
    pub db_type: DatabaseType,
    pub limit: Option<DMLLimit>,
    pub exclude_columns: Option<Vec<String>>,
    pub obfuscations: Option<Obfuscation>,

}
