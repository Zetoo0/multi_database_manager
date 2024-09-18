use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Procedure{
    pub name: String,
    pub definition: String,
}