use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Procedure{
    pub name: String,
    pub definition: String,
}