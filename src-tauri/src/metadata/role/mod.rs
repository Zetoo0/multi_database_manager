use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Role{
    pub name: String,
    pub is_super: Option<bool>,
    pub is_insherit: Option<bool>,
    pub is_create_role: Option<bool>,
    pub is_create_db: Option<bool>,
    pub can_login: Option<bool>,
    pub is_replication: Option<bool>,
    
}