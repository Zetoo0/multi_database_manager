use crate::domain::datb::database_type::DatabaseType;
use crate::domain::metadata::column::Column;
use rbs::value::map::ValueMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMLLimit {
    pub colname: String,
    pub limit: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObfuscationType {
    FIXED,
    REPLACE,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obfuscation {
    pub type_: ObfuscationType,
    pub col_name: Vec<String>,
}

pub trait GenerateDML: Send + Sync {
    async fn to_insert(
        &self,
        db_type: DatabaseType,
        rows: Vec<ValueMap>,
        columns: HashMap<String, Column>,
        db_name: &str,
        limit: Option<DMLLimit>,
        exclude_columns: Option<Vec<String>>,
        obfuscations: Option<Obfuscation>,
    ) -> Result<String, String>;
    fn to_update(&self, db_type: DatabaseType) -> Result<String, String>;
    fn to_delete(&self, db_type: DatabaseType) -> Result<String, String>;
}
