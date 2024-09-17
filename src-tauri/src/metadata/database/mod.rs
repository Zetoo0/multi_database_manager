use crate::function::Function;
use crate::procedure::Procedure;
use crate::role::Role;
use crate::table::Table;
use crate::view::View;

pub struct Database{
    pub name: String,
    pub functions: Option<Vec<Function>>,
    pub procedures: Option<Vec<Procedure>>,
    pub roles: Vec<Role>,
    pub tables: Vec<Table>,
    pub views: Vec<View>
}