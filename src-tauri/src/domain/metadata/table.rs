use crate::domain::datb::data_type_map::data_type_map;
use crate::domain::datb::database_type::DatabaseType;
use crate::domain::datb::dml_generate::DMLLimit;
use crate::domain::datb::dml_generate::GenerateDML;
use crate::domain::datb::dml_generate::Obfuscation;
use crate::domain::io::filewrite::write_out_all_into_one_file_at_once;
use crate::domain::io::filewrite::write_out_with_chunks;
use crate::domain::metadata::constraint::Constraint;
use crate::domain::metadata::index::Index;
use crate::domain::metadata::rls_policy::RlsPolicy;
use crate::domain::metadata::rule::Rule;
use crate::domain::metadata::trigger::Trigger;
use crate::domain::obfuscation::obfuscations::fixed_obfuscation;
use crate::domain::obfuscation::obfuscations::name_obfuscate;
use crate::domain::{datb::ddl_generate::GenerateDDL, metadata::column::Column};
use rbs::value::map::ValueMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use futures::executor::block_on;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Option<HashMap<String, Column>>,
    pub constraints: Option<HashMap<String, Constraint>>,
    pub indexes: Option<HashMap<String, Index>>,
    pub triggers: Option<HashMap<String, Trigger>>,
    pub rules: Option<HashMap<String, Rule>>,
    pub rls_policies: Option<HashMap<String, RlsPolicy>>,
    pub db_name: String,
    pub schema_name: Option<String>,
    pub type_: Option<String>,
}

struct DDLResult {
    data: String,
}
struct DDLError {
    message: String,
}

impl GenerateDDL for Table {
    fn to_create_function(&self, target_db_type: DatabaseType) -> Result<String, String> {
        log::info!("to create for {}, target_db: {}", self.name, target_db_type);
        let mut ddl = format!("CREATE TABLE {} (\n", self.name);
        if let Some(cols) = &self.columns {
            for (col_name, col) in cols {
                ddl.push_str(&format!(
                    "{} {} {},\n",
                    col_name,
                    if col.is_primary_key.unwrap() {
                        match target_db_type {
                            DatabaseType::MySql => format!(
                                "{} AUTO_INCREMENT PRIMARY KEY",
                                col.data_type.as_ref().unwrap()
                            ),
                            DatabaseType::MsSql => format!(
                                "{} PRIMARY KEY IDENTITY(1,1)",
                                col.data_type.as_ref().unwrap()
                            ),
                            DatabaseType::Sqlite => {
                                format!("{} INTEGER PRIMARY KEY", col.data_type.as_ref().unwrap())
                            }
                            DatabaseType::Oracle => {
                                format!("{} GENERATED AS IDENTITY", col.data_type.as_ref().unwrap())
                            }
                            DatabaseType::Postgres => {
                                format!("{} SERIAL PRIMARY KEY", col.data_type.as_ref().unwrap())
                            }
                        }
                    } else {
                        let mut type_definition = String::new();
                        // Handle data types and lengths if any
                        if col.maximum_length.clone().unwrap() != 0 {
                            type_definition.push_str(
                                format!(
                                    "{}({})",
                                    data_type_map(col.data_type.as_ref().unwrap(), target_db_type),
                                    col.maximum_length.as_ref().unwrap()
                                )
                                .as_str(),
                            );
                        } else {
                            log::info!("MAPPING DATA TYPE: {}", col.data_type.as_ref().unwrap());
                            type_definition.push_str(
                                data_type_map(col.data_type.as_ref().unwrap(), target_db_type)
                                    .as_str(),
                            );
                        }

                        format!(
                            "{} {}",
                            type_definition,
                            if col.is_nullable.unwrap_or(true) {
                                "NULL"
                            } else {
                                "NOT NULL"
                            }
                        )
                    },
                    if let Some(def_val) = &col.default_value {
                        if def_val == "CURRENT_TIMESTAMP" {
                            "DEFAULT GETDATE()".to_string()
                        } else if def_val.contains("nextval") {
                            "".to_string()
                        } else {
                            format!("DEFAULT {}", def_val)
                        }
                    } else {
                        "".to_string()
                    }
                ));
            }
            ddl.pop(); // Remove the last comma
            ddl.pop(); // Remove the last newline
        }
        ddl.push_str("\n);"); // Close the statement

        Ok(ddl)
    }
}

impl GenerateDML for Table {
    async fn to_insert(
        &self,
        db_type: DatabaseType,
        rows: Vec<ValueMap>,
        columns: HashMap<String, Column>,
        db_name: &str,
        limit: Option<DMLLimit>,
        exclude_columns: Option<Vec<String>>,
        obfuscations: Option<Obfuscation>,
    ) -> Result<String, String> {
        log::info!("to insert for {}, target_db: {}, limit: {:?}, exclude_columns: {:?}, obfuscations: {:?}",self.name,db_type,limit,exclude_columns,obfuscations);
        //Cloning the columns without the excluded columns
        /*let mut column_clone:HashMap<String,Column> = self.columns
        .as_ref()
        .unwrap()
        .iter()
        .filter(|key| !exclude_columns.as_ref().unwrap().as_slice().contains(key.0))
        .map(|(k,v)| (k.to_string(),v.clone()))
        .collect();*/
        let obfuscation_clone = obfuscations.clone();
        //Find the primary key columns
        let primary_key_column = self
            .columns
            .as_ref()
            .unwrap()
            .values()
            .find(|col| col.is_primary_key.unwrap_or(false));
        //Get the rows from the database
        /*let rows = repo.get_row_value(
        &self.name,
        db_name, &column_clone.keys().map(|v| v.to_string())
          .collect::<Vec<_>>()).await.unwrap();*/
        let col_keys = columns
            .keys()
            .map(|k| format!("`{}`", k))
            .collect::<Vec<_>>()
            .join(", ");
        let mut insert_vec: Vec<String> = Vec::new();
        let mut col_values: String = String::from("a");
        let mut row_namaes: String = String::from("b");
        let mut column_value_transform_response = String::new();
        for row in rows {
            col_values.clear();
            row_namaes.clear();
            for (row_name, row_value) in row {
                let mut r_name = row_name.to_string();
                r_name = r_name.replace("\"", "");
                let curr_column = columns
                    .get(&r_name)
                    .unwrap_or_else(|| panic!("column not found: {}", r_name))
                    .clone();
                if curr_column.data_type.unwrap() == "date".to_string() {
                    column_value_transform_response = transform_col_value(
                        &row_value.to_string(),
                        true,
                        obfuscation_clone.clone(),
                        db_type,
                        r_name.clone(),
                    ).await;
                } else {
                    column_value_transform_response = transform_col_value(
                        &row_value.to_string(),
                        false,
                        obfuscation_clone.clone(),
                        db_type,
                        r_name.clone(),
                    ).await;
                }
                log::info!("Obfusc Column value: {}", column_value_transform_response);
                col_values.push_str(&column_value_transform_response);
                match db_type {
                    DatabaseType::MsSql => {
                        row_namaes.push_str(format!("[{}],", r_name).as_str());
                    }
                    _ => {
                        row_namaes.push_str(format!("`{}`,", r_name).as_str());
                    }
                }
            }
            println!("Column values end: {}", col_values.clone());
            row_namaes.pop();
            col_values.pop();
            insert_vec.push(format!(
                "INSERT INTO {} ({}) VALUES ({});",
                self.name, row_namaes, col_values
            ));
        }
        log::info!("INSERTS TABLE: {:?}", insert_vec);
        if let Some(limit) = limit {
            write_out_with_chunks(insert_vec, self.name.clone(), limit.limit, true).await;
        } else {
            write_out_all_into_one_file_at_once(insert_vec, self.name.clone(), true).await;
        }
        Ok("Success".to_string())
    }
    fn to_update(&self, db_type: DatabaseType) -> Result<String, String> {
        todo!()
    }
    fn to_delete(&self, db_type: DatabaseType) -> Result<String, String> {
        todo!()
    }
}

/// Transform the date to the format required by the database
pub fn convert_table_date_row(row: String, db_type: DatabaseType) -> String {
    let mut row_value: String = row.clone();
    row_value = row_value.replace("-", "");
    row_value = row_value.replace(".", "");
    match db_type {
        DatabaseType::MySql => {
            return format!(
                "STR_TO_DATE(CONCAT(DATE_FORMAT({}, '%Y%m%d')), '%Y%m%d')",
                row_value
            )
        }
        DatabaseType::Postgres => {
            return format!(
                "STR_TO_DATE(CONCAT(DATE_FORMAT({}, '%Y%m%d')), '%Y%m%d')",
                row_value
            )
        }
        DatabaseType::MsSql => {
            return format!("CONVERT(DATE,FORMAT({},'yyyyMMDD'),112)", row_value)
        }
        DatabaseType::Sqlite => return format!("DATE(strftime('%Y%m%d',{})", row_value),
        DatabaseType::Oracle => {
            return format!("TO_DATE(TO_CHAR({}, 'YYYYMMDD'), 'YYYYMMDD')", row_value)
        }
    }
}

/// Transform and obfuscate the column value it has to
pub async fn transform_col_value(
    value: &str,
    is_date: bool,
    obfuscations: Option<Obfuscation>,
    db_type: DatabaseType,
    row_name: String,
) -> String {
    if is_date {
        if obfuscations.is_some() && obfuscations.as_ref().unwrap().col_name.contains(&row_name) {
            log::info!("OBFUSCATING DA: {}", value);
            return format!("{:?},", name_obfuscate(value).await.unwrap());
        } else {
            return format!("{}, ", convert_table_date_row(value.to_string(), db_type));
        }
    } else {
        if obfuscations.is_some() && obfuscations.as_ref().unwrap().col_name.contains(&row_name) {
            log::info!("String with obfuscation");
            let word = name_obfuscate(value).await;
            log::info!("Obfuscated word returning: {:?}", word.as_ref());
            return format!("{:?},", word.unwrap());
        } else {
            return format!("{},", value);
        }
    }
}
