use crate::domain::datb::database_type::DatabaseType;
use crate::domain::datb::ddl_generate::GenerateDDL;
use crate::domain::io::filewrite::write_out_into_one_file;
use crate::domain::metadata::aggregate::Aggregate;
use crate::domain::metadata::catalog::Catalog;
use crate::domain::metadata::{
    constraint::Constraint, foreign_data_wrapper::ForeignDataWrapper, function::Function,
    lock::Lock, materalized_view::MateralizedView, procedure::Procedure, sequence::Sequence,
    table::Table, trigger::Trigger, utype::Type, view::View,
};
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use super::role::Role;
use super::trigger::TriggerFunction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schema {
    pub name: String,
    pub functions: Option<HashMap<String, Function>>,
    pub procedures: Option<HashMap<String, Procedure>>,
    pub tables: Option<HashMap<String, Table>>,
    pub views: Option<HashMap<String, View>>,
    pub constraints: Option<HashMap<String, Constraint>>,
    pub locks: Option<HashMap<String, Lock>>,
    pub triggers: Option<HashMap<String, TriggerFunction>>,
    pub types: Option<HashMap<String, Type>>,
    pub aggregates: Option<HashMap<String, Aggregate>>,
    pub materalized_views: Option<HashMap<String, MateralizedView>>,
    pub catalogs: Option<HashMap<String, Catalog>>,
    pub sequences: Option<HashMap<String, Sequence>>,
    pub roles: Option<HashMap<String,Role>>,
    pub type_: Option<String>,
}

impl Schema {
    pub async fn generate_ddls(&self, target_db_type: DatabaseType) {
        if let (
            Some(functions),
            Some(tables),
            Some(views),
            Some(sequences),
            Some(types),
            Some(procedures),
        ) = (
            &self.functions,
            &self.tables,
            &self.views,
            &self.sequences,
            &self.types,
            &self.procedures,
        ) {
            let mut index_ddls: Vec<String> = Vec::new();

            let function_ddls: Vec<String> = functions
                .iter()
                .map(|(_, v)| v.to_create_function(target_db_type))
                .filter_map(Result::ok)
                .collect();

            let table_ddls: Vec<String> = tables
                .iter()
                .map(|(_, v)| v.to_create_function(target_db_type).unwrap())
                .collect();

            let view_ddls: Vec<String> = views
                .iter()
                .map(|(_, v)| v.to_create_function(target_db_type).unwrap())
                .collect();

            let seq_ddls: Vec<String> = sequences
                .iter()
                .map(|(_, v)| v.to_create_function(target_db_type).unwrap())
                .collect();

            let type_ddls: Vec<String> = types
                .iter()
                .map(|(_, v)| v.to_create_function(target_db_type).unwrap())
                .collect();

            let procedure_ddls: Vec<String> = procedures
                .iter()
                .map(|(_, v)| v.to_create_function(target_db_type).unwrap())
                .collect();

            // Work with index DDLs (from a specific table)
            if let Some(product_table) = tables.get("product") {
                if let Some(indexes) = &product_table.indexes {
                    index_ddls = indexes
                        .iter()
                        .map(|(_, v)| v.to_create_function(target_db_type).unwrap())
                        .collect();
                }
            }

            write_out_into_one_file(function_ddls, Path::new("DDL").join("create-function.sql"))
                .await;
            write_out_into_one_file(table_ddls, Path::new("DDL").join("create-table.sql")).await;
            write_out_into_one_file(view_ddls, Path::new("DDL").join("create-view.sql")).await;
            write_out_into_one_file(seq_ddls, Path::new("DDL").join("create-sequence.sql")).await;
            write_out_into_one_file(type_ddls, Path::new("DDL").join("create-type.sql")).await;
            write_out_into_one_file(index_ddls, Path::new("DDL").join("create-index.sql")).await;
            write_out_into_one_file(
                procedure_ddls,
                Path::new("DDL").join("create-procedure.sql"),
            )
            .await;
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
    pub name: String,
    pub functions: Option<HashMap<String, Function>>,
    pub procedures: Option<HashMap<String, Procedure>>,
    pub tables: Option<HashMap<String, Table>>,
    pub views: Option<HashMap<String, View>>,
    pub constraints: Option<HashMap<String, Constraint>>,
    pub locks: Option<HashMap<String, Lock>>,
    pub triggers: Option<HashMap<String, Trigger>>,
    pub types: Option<HashMap<String, Type>>,
    pub aggregates: Option<HashMap<String, Aggregate>>,
    pub materalized_views: Option<HashMap<String, MateralizedView>>,
    pub catalogs: Option<HashMap<String, Catalog>>,
    pub sequences: Option<HashMap<String, Sequence>>,
    pub foreign_data_wrappers: Option<HashMap<String, ForeignDataWrapper>>,
}
