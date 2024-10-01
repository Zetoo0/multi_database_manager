use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Lock{
    name:String,
}