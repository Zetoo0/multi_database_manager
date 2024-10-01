use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Rule{
    pub name: String,
    pub definition: String,
}