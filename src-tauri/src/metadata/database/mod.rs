use std::collections::HashMap;

use serde::{Serialize,Deserialize};
use crate::metadata::{column::Column,constraint::Constraint,foreign_data_wrapper::ForeignDataWrapper,function::Function,lock::Lock,materalized_view::MateralizedView,procedure::Procedure,table::Table,trigger::Trigger,user::User,utype::Type,view::View,sequence::Sequence};
use crate::metadata::aggregate::Aggregate;
use crate::metadata::rls_policy::RlsPolicy;
use crate::metadata::catalog::Catalog;
use dashmap::DashMap;
use dashmap::DashMap;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Database{
    pub name: String,
    pub functions: Option<HashMap<String,Function>>,
    pub procedures: Option<HashMap<String,Procedure>>,
    pub tables: Option<HashMap<String, Table>>,
    pub views: Option<HashMap<String, View>>,
    pub constraints: Option<HashMap<String, Constraint>>,
    pub foreign_data_wrappers: Option<HashMap<String, ForeignDataWrapper>>,
    pub locks: Option<HashMap<String, Lock>>,
    pub triggers: Option<HashMap<String, Trigger>>,
    pub types: Option<HashMap<String, Type>>,
    pub aggregates: Option<HashMap<String, Aggregate>>,
    pub materalized_views: Option<HashMap<String, MateralizedView>>,
    pub catalogs: Option<HashMap<String, Catalog>>,
    pub sequences: Option<HashMap<String,Sequence>>
}
