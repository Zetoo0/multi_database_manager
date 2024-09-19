use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Database{
    pub name: String,
    /*pub functions: Option<Vec<Function>>,
    pub procedures: Option<Vec<Procedure>>,
    pub roles: Option<Vec<Role>>,
    pub tables: Option<Vec<Table>>,
    pub views: Option<Vec<View>>*/
}