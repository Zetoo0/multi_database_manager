use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Clone,PartialEq)]
pub struct Role {
    pub name: String,
    pub is_super: Option<bool>,
    pub is_insherit: Option<bool>,
    pub is_create_role: Option<bool>,
    pub is_create_db: Option<bool>,
    pub can_login: Option<bool>,
    pub is_replication: Option<bool>,
    pub connection_limit: Option<i32>,
    pub valid_until: Option<String>,
    pub password: Option<String>,
    pub db_name: String,
    pub schema_name: String,
    pub type_: String,
}