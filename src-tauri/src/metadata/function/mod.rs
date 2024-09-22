use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Function{
    pub name: String,
    pub definition: String,
}