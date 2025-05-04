use crate::domain::datb::data_type_map::data_type_map;
use crate::domain::datb::database_type::DatabaseType;
use crate::domain::datb::ddl_generate::GenerateDDL;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub definition: String,
    pub full_function: Option<String>,
    pub parameters: Option<Vec<String>>,
    pub return_type: Option<String>,
    pub type_: Option<String>,
    pub schema_name: Option<String>,
    pub db_name: String,
}

impl GenerateDDL for Function {
    fn to_create_function(&self, target_db_type: DatabaseType) -> Result<String, String> {
        match target_db_type {
            DatabaseType::MySql => {
                let mut converted_param = String::new();
                for param in self.parameters.as_ref().unwrap() {
                    let mut param_split = param.split(' ').collect::<Vec<&str>>();
                    if param_split.len() == 4 {
                        param_split.remove(0);
                    }
                    match param_split.len() {
                        2 => {
                            let data_name = param_split[0].replace("@", "");
                            let data_type = data_type_map(param_split[1], target_db_type);

                            converted_param
                                .push_str(format!("{} {}", data_name, data_type).as_str());
                        }
                        _ => {
                            return Err("Invalid parameter".to_string());
                        }
                    }
                }
                let ddl = format!(
                    "CREATE OR REPLACE FUNCTION {} ({});",
                    self.name, converted_param
                );
                Ok(ddl)
            }
            DatabaseType::Postgres => {
                let mut converted_param = String::new();
                for param in self.parameters.as_ref().unwrap() {
                    let mut param_split = param.split(' ').collect::<Vec<&str>>();
                    if param_split.len() == 4 {
                        param_split.remove(0);
                    }
                    match param_split.len() {
                        2 => {
                            let data_name = param_split[0].replace("@", "");
                            let data_type = data_type_map(param_split[1], target_db_type);

                            converted_param
                                .push_str(format!("{} ({}),", data_name, data_type).as_str());
                        }
                        _ => {
                            return Err("Invalid parameter".to_string());
                        }
                    }
                }
                let ddl = format!(
                    "CREATE OR REPLACE FUNCTION {} ({});",
                    self.name, converted_param
                );
                Ok(ddl)
            }
            DatabaseType::MsSql => {
                let mut converted_param = String::new();
                for param in self.parameters.as_ref().unwrap() {
                    let mut param_split = param.split(' ').collect::<Vec<&str>>();
                    if param_split.len() == 4 {
                        param_split.remove(0);
                    }
                    match param_split.len() {
                        2 => {
                            let data_name = param_split[0];
                            let data_type = data_type_map(param_split[1], target_db_type);
                            converted_param
                                .push_str(format!("@{} {},", data_name, data_type).as_str());
                        }
                        _ => {
                            return Err("Invalid parameter".to_string());
                        }
                    }
                }
                let ddl = format!("CREATE FUNCTION {} ({});", self.name, converted_param);
                Ok(ddl)
            }
            DatabaseType::Oracle => {
                let mut converted_param = String::new();
                for param in self.parameters.as_ref().unwrap() {
                    let mut param_split = param.split(' ').collect::<Vec<&str>>();
                    if param_split.len() == 4 {
                        param_split.remove(0);
                    }
                    match param_split.len() {
                        2 => {
                            let data_name = param_split[0].replace("@", "");
                            let data_type = data_type_map(param_split[1], target_db_type);

                            converted_param
                                .push_str(format!("{} IN {},", data_name, data_type).as_str());
                        }
                        _ => {
                            return Err("Invalid parameter".to_string());
                        }
                    }
                }
                let ddl = format!("CREATE FUNCTION {} ({});", self.name, converted_param);
                Ok(ddl)
            }
            DatabaseType::Sqlite => Err("Sqlite does not support procedure".to_string()),
        }
    }
}
