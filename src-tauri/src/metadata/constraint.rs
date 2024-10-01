use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Constraint{
    pub name:String,
    pub c_type:String,
}