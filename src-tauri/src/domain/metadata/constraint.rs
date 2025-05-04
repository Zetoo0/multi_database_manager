use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone,PartialEq)]
pub struct Constraint {
    pub name: String,
    pub c_type: String,
    pub table_name: String,
    pub column_name: String,
    pub fk_table: String,
    pub fk_column: String,
    pub db_name: String,
    pub schema_name: Option<String>,
    pub type_:String,

}