use rbs::value::map::ValueMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryInfo {
    pub sql: String,
    pub db_type: String,
    pub db_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub rows: Vec<ValueMap>,
}
