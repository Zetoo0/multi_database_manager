// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod metadata;
pub mod repository;
pub mod service;
use std::{borrow::Borrow, fmt::format, future::IntoFuture, str::FromStr};
use metadata::{table};
use rbdc::db::{self, ConnectOptions, Connection, Driver, Placeholder, Row};
use rbdc_mssql::MssqlDriver;
use rbdc_mysql::MysqlDriver;
use rbdc_pg:: PgDriver;
use rbdc_sqlite::SqliteDriver;
use rbs::Value;
use serde::{Serialize,Deserialize}; 
use strum_macros::{Display, EnumString};
use fast_log::{init, plugin::file, Config};
use log::{info,warn,error};
use std::sync::Arc;

use crate::repository::postgres_repository::PostgresRepository;
use crate::repository::database_repository::DatabaseRepository;
use crate::repository::mysql_repository::MySqlRepository;
use crate::repository::mssql_repository::MsSqlRepository;



#[derive(Serialize,Deserialize,Debug,EnumString,Display)]
enum DriverType{
  #[strum(serialize="mysql")]
  Mysql,
  #[strum(serialize="postgresql")]
  Pg,
  #[strum(serialize="mssql")]
  Mssql,
  #[strum(serialize="sqlite")]
  Sqlite
}
 
fn get_driver(driver_type:DriverType)->Box<dyn Driver>{
  match driver_type{
    DriverType::Mysql => Box::new(MysqlDriver {}),
    DriverType::Pg => Box::new(PgDriver {}),
    DriverType::Mssql => Box::new(MssqlDriver {}),
    DriverType::Sqlite => Box::new(SqliteDriver {}),
  }
}

#[derive(Serialize, Deserialize,Debug,Clone)]
struct DatabaseConnection{
  port: String,
  server: String,
  username: String,
  password: String,
  driver_type: String,
}

#[tauri::command]
async fn repo_test(db_connection:DatabaseConnection) -> Result<String,String> {
  log::info!("Repository");

  match db_connection.driver_type.as_str(){
    "postgresql" => {
      let postgres = Arc::new(PostgresRepository::new(db_connection));
      let databases = postgres.get_databases().await.unwrap();
      
      let mut tables:Value = Value::Null;
      let mut columns:Value = Value::Null;
      for db_name in databases{
        println!("in the for?");
        for dab in db_name.1{
          println!("{:?}",dab.1.as_str().unwrap());
          tables = postgres.get_tables(&dab.1.as_str().unwrap()).await.unwrap();
          //println!("Tablet len? {:?}",tables);
          println!("Database: {:?}",dab.1.as_str().unwrap());
          for table in tables{
            println!("tables doko?");
            for t in table.1{
              println!("Table: {:?}",t.1.as_str().unwrap());
              columns = postgres.get_columns(dab.1.as_str().unwrap(), t.1.as_str().unwrap()).await.unwrap();
              println!("Columns: {:?}",columns);
              println!("Partitions: {:?}",postgres.get_partitions(dab.1.as_str().unwrap(), t.1.as_str().unwrap()).await.unwrap());
              println!("Constraints: {:?}",postgres.get_constraints(&dab.1.as_str().unwrap(),t.1.as_str().unwrap_or_default()).await.unwrap());
               println!("Indexes: {:?}",postgres.get_indexes(&dab.1.as_str().unwrap(),t.1.as_str().unwrap_or_default()).await.unwrap());
            }
          }
          println!("Views: {:?}",postgres.get_views(&dab.1.as_str().unwrap()).await.unwrap());
          println!("Functions: {:?}",postgres.get_functions(&dab.1.as_str().unwrap()).await.unwrap());
          println!("Procedures: {:?}",postgres.get_stored_procedures(&dab.1.as_str().unwrap()).await.unwrap());
          println!("Triggers: {:?}",postgres.get_trigger_functions(&dab.1.as_str().unwrap()).await.unwrap());
          println!("Sequences: {:?}",postgres.get_sequences(&dab.1.as_str().unwrap()).await.unwrap());
          println!("Roles and users: {}",postgres.get_roles_and_users(&dab.1.as_str().unwrap()).await.unwrap());
          println!("languages: {:?}",postgres.get_languages(&dab.1.as_str().unwrap()).await.unwrap());
          println!("Schemas: {:?}",postgres.get_schemas(&dab.1.as_str().unwrap()).await.unwrap());
          println!("Active Sessions: {:?}",postgres.get_active_sessions().await.unwrap());
          println!("Materalized Views: {:?}",postgres.get_materalized_views(&dab.1.as_str().unwrap()).await.unwrap());
          println!("Event Triggers: {:?}", postgres.get_event_triggers(&dab.1.as_str().unwrap()).await.unwrap());
          println!("Types: {:?}", postgres.get_types(&dab.1.as_str().unwrap()).await.unwrap());
          println!("Foreign Data Wrappers: {:?}", postgres.get_foreign_data_wrappers(&dab.1.as_str().unwrap()).await.unwrap());
          println!("Locks: {:?}", postgres.get_locks(&dab.1.as_str().unwrap()).await.unwrap());
          println!("-------------------------------------------------");
        }
      }
      println!("Table product after initialization: {:?}",postgres.get_tables("products").await.unwrap());
      println!("Table product columns after initialization: {:?}",postgres.get_columns("products", "product").await.unwrap());
      return Ok(String::from("Successfully connected!"));
    },
    "mysql" => {
      let _mysql_ = Arc::new(MySqlRepository::new(db_connection));
      let databases = _mysql_.get_databases().await.unwrap();
      let mut tables = Value::Null;
      let mut columns:Value = Value::Null;
      for db_name in databases{
        for db in db_name.1{
          tables = _mysql_.get_tables(db.1.as_str().unwrap()).await.unwrap();
          println!("tables: {:?}",tables);
          for table in tables{
            for t in table.1{
              println!("Table: {:?}",t.1.as_str().unwrap());
              columns = _mysql_.get_columns(db.1.as_str().unwrap(), t.1.as_str().unwrap()).await.unwrap();
              println!("Columns: {:?}",columns);
              println!("Indexes: {:?}",_mysql_.get_indexes(&db.1.as_str().unwrap(),t.1.as_str().unwrap_or_default()).await.unwrap());
              println!("Constraints: {:?}",_mysql_.get_constraints(&db.1.as_str().unwrap(),t.1.as_str().unwrap_or_default()).await.unwrap());
            }
          }
         // println!("Views: {:?}",_mysql_.get_views(&db.1.as_str().unwrap()).await.unwrap());
         // println!("Functions: {:?}",_mysql_.get_functions(&db.1.as_str().unwrap()).await.unwrap());
          //println!("Procedures: {:?}",_mysql_.get_stored_procedures(&db.1.as_str().unwrap()).await.unwrap());
        }
      }
      return Ok(String::from("Successfully connected!"));
    },
    "mssql" => {
      let _mssql_ = Arc::new(MsSqlRepository::new(db_connection));
      let databases = _mssql_.get_databases().await.unwrap();
      let mut tables = Value::Null;
      for db_name in databases{
        for db in db_name.1{
          tables = _mssql_.get_tables(db.1.as_str().unwrap()).await.unwrap();
          println!("tables: {:?}",tables);
        }
      }
      return Ok(String::from("Successfully connected!"));
    },
    _ => {
      return Err(String::from("Not implemented"));
    }
  }

}

fn main() {
    fast_log::init(Config::new().file("logs/log_info")).unwrap();
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![repo_test])
    .run(tauri::generate_context!())    
    .expect("error while running tauri application");
}
