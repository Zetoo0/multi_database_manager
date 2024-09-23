use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct MateralizedView{
    pub name:String,
    pub definition:String,
}