use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct User{
    pub name: String,
    pub role: String,
}