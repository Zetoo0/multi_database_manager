use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Function{
    pub name: String,
    pub definition: String,
}