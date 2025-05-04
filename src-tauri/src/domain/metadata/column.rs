use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Column {
    pub name: String,
    pub data_type: Option<String>,
    pub is_nullable: Option<bool>,
    pub default_value: Option<String>,
    pub is_primary_key: Option<bool>,
    pub maximum_length: Option<i64>,
    pub schema_name: Option<String>,
    pub table_name: String,
    pub db_name: String,
    pub type_: String,
}
