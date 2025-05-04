use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trigger {
    pub name: String,
    pub definition: String,
    pub type_: String,
    pub db_name: String,
    pub schema_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TriggerFunction {
    pub name: String,
   // pub function_name: String,
    pub definition: String,
    pub type_: String,
    pub db_name: String,
    pub schema_name: String,
}