use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct View{
    pub name: String,
    pub definition: String,
}