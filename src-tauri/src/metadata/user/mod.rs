use crate::role::Role;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct User{
    pub name: String,
    pub role: Option<Role>,
}