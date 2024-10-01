use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct View{
    pub name: String,
    pub definition: String,
}