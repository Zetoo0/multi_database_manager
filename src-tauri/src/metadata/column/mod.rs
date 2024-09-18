use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Column{
    pub name: String,
    pub data_type: Option<String>,
    pub is_nullable: Option<bool>,
    pub default_value: Option<Sting>
}