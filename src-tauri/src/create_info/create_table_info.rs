use serde::{Deserialize, Serialize};

use crate::metadata::column::Column;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct CreateTableInfo{
    pub table_name: String,
    pub columns: Vec<Column>,
}