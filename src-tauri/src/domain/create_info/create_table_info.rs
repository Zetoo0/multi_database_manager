use serde::{Deserialize, Serialize};

use crate::domain::metadata::{column::Column, view::View};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTableInfo {
    pub table_name: String,
    pub columns: Vec<Column>,
    pub db_name: String,
    pub schema_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIndexInfo {
    pub table_name: String,
    pub index_name: String,
    pub columns: Vec<String>,
    pub schema_name: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateViewInfo {
    pub view_name: String,
    pub columns: Vec<String>,
    pub table_name: String,
    pub stmt: String,
    pub schema_name: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSequenceInfo {
    pub sequence_name: String,
    pub start_val: String,
    pub minimum_val: String,
    pub maximum_val: String,
    pub increment: String,
    pub cycle: bool,
    pub schema_name: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFunctionInfo {
    pub function_name: String,
    pub params: Vec<String>,
    pub stmt: String,
    pub schema_name: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSchemaInfo {
    schema_name: String,
    db_name: String,
    user_name: Option<String>,
}

/*#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateViewInfo {
    view: View,
    db_name: String,
}*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTriggerInfo {
    pub name: String,
    pub when: String,
    pub type_: String,
    pub table_name: String,
    pub function_name: String,
    pub database_name: String,
}