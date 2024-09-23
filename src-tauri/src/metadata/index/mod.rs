use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Index{
    pub name: String,
    pub definition: String,
}