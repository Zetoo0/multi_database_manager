use crate::domain::datb::data_type_map::data_type_map;
use crate::domain::datb::database_type::DatabaseType;
use crate::domain::datb::ddl_generate::GenerateDDL;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Type {
    pub name: String,
    pub fields: Option<Vec<TypeField>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TypeField {
    pub name: String,
    pub type_name: String,
}

impl fmt::Display for TypeField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.name, self.type_name)
    }
}

impl GenerateDDL for Type {
    fn to_create_function(&self, target_db_type: DatabaseType) -> Result<String, String> {
        let mut ddl = String::new();
        match target_db_type {
            DatabaseType::Postgres => {
                ddl.push_str(
                    format!(
                        "CREATE TYPE {} AS ({});",
                        self.name,
                        self.fields
                            .clone()
                            .unwrap()
                            .iter()
                            .map(|field| format!(
                                "{} {}",
                                field.name,
                                data_type_map(&field.type_name, DatabaseType::Postgres)
                            ))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                    .as_str(),
                );
            }
            DatabaseType::MySql => {
                ddl.push_str(
                    format!(
                        "CREATE TABLE {} ({});",
                        self.name,
                        self.fields
                            .clone()
                            .unwrap()
                            .iter()
                            .map(|field| format!(
                                "{} {}",
                                field.name,
                                data_type_map(&field.type_name, DatabaseType::MySql)
                            ))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                    .as_str(),
                );
            }
            DatabaseType::MsSql => {
                ddl.push_str(
                    format!(
                        "CREATE TYPE {} AS TABLE({});",
                        self.name,
                        self.fields
                            .clone()
                            .unwrap()
                            .iter()
                            .map(|field| format!(
                                "{} {}",
                                field.name,
                                data_type_map(&field.type_name, DatabaseType::MsSql)
                            ))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                    .as_str(),
                );
            }
            DatabaseType::Sqlite => {
                return Err("Sqlite does not support types".to_string());
            }
            DatabaseType::Oracle => {
                ddl.push_str(
                    format!(
                        "CREATE OR REPLACE TYPE {} AS OBJECT ({});",
                        self.name,
                        self.fields
                            .clone()
                            .unwrap()
                            .iter()
                            .map(|field| format!(
                                "{} {}",
                                field.name,
                                data_type_map(&field.type_name, DatabaseType::Oracle)
                            ))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                    .as_str(),
                );
            }
        }
        Ok(ddl)
    }
}
