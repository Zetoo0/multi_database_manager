// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


 
pub mod database;
pub mod metadata;

use std::{borrow::Borrow, fmt::format, future::IntoFuture, str::FromStr};
use metadata::{repository::DatabaseRepository, table};
use rbdc::db::{Driver,Connection,ConnectOptions,Row,Placeholder};
use rbdc_mssql::MssqlDriver;
use rbdc_mysql::MysqlDriver;
use rbdc_pg::{column, types::decode::Decode, value::PgValue, PgDriver};
use rbdc_sqlite::SqliteDriver;
use rbatis::{executor::{Executor, RBatisRef}, Error}; 
use rbs::Value;
use serde::{Serialize,Deserialize}; 
use serde_json::Value as SerdeValue;
use strum_macros::{Display, EnumString, ToString};
use fast_log::{init, plugin::file, Config};
use log::{info,warn,error};

use crate::metadata::repository::PostgresRepository;



#[derive(Serialize,Deserialize,Debug,EnumString,Display)]
enum DriverType{
  #[strum(serialize="mysql")]
  Mysql,
  #[strum(serialize="postgres")]
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

#[derive(Serialize, Deserialize)]
struct DatabaseConnection{
  port: String,
  server: String,
  username: String,
  password: String,
  driver_type: String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Database{
    pub name: String,
    /*pub functions: Option<Vec<Function>>,
    pub procedures: Option<Vec<Procedure>>,
    pub roles: Option<Vec<Role>>,
    pub tables: Option<Vec<Table>>,
    pub views: Option<Vec<View>>*/
}

#[tauri::command]
async fn init_database(data:DatabaseConnection) -> Result<String, String> {
  let rb = rbatis::RBatis::new();
  match DriverType::from_str(&data.driver_type){
    Ok(variante) => {
      let url = format!("{}://{}:@{}:{}",variante.to_string(),data.username,data.server,data.port);
      let res = rb.link(get_driver(variante), &url);
      println!("{:?}",res.await.is_ok());
      println!("{:?}",rb.driver_type());
      let query_res = rb.query("SELECT * FROM pg_roles;",vec![]);
      println!("{:?}",query_res.await.map(|rows| rows.into_iter().map(|row| row.1)));
      Ok(String::from("Successfully conneced!"))
    },
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
async fn repo_test() -> Result<String,String> {
  log::info!("Repository");
  let rb = rbatis::RBatis::new();
  let postgres = PostgresRepository::PostgresRepository::new();
  let databases = postgres.get_databases().await.unwrap();
  let mut tables:Value = Value::Null;
  let mut columns:Value = Value::Null;
  for db_name in databases{
    for dab in db_name.1{
      tables = postgres.get_tables(&dab.1.as_str().unwrap()).await.unwrap();
      println!("Tablet len? {:?}",tables);
      println!("Database: {:?}",dab.1.as_str().unwrap());
      for table in tables{
        println!("tables doko?");
        for t in table.1{
          println!("Table: {:?}",t.1.as_str().unwrap());
          columns = postgres.get_columns(dab.1.as_str().unwrap(), t.1.as_str().unwrap()).await.unwrap();
          println!("Columns: {:?}",columns);
          println!("Partitions: {:?}",postgres.get_partitions(dab.1.as_str().unwrap(), t.1.as_str().unwrap()).await.unwrap());
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
      println!("Indexes: {:?}",postgres.get_indexes(&dab.1.as_str().unwrap()).await.unwrap());
      println!("Active Sessions: {:?}",postgres.get_active_sessions().await.unwrap());
      println!("Materalized Views: {:?}",postgres.get_materalized_views(&dab.1.as_str().unwrap()).await.unwrap());
      println!("Event Triggers: {:?}", postgres.get_event_triggers(&dab.1.as_str().unwrap()).await.unwrap());
      println!("Types: {:?}", postgres.get_types(&dab.1.as_str().unwrap()).await.unwrap());
      println!("Foreign Data Wrappers: {:?}", postgres.get_foreign_data_wrappers(&dab.1.as_str().unwrap()).await.unwrap());
      println!("Constraints: {:?}",postgres.get_constraints(&dab.1.as_str().unwrap()).await.unwrap());
      println!("Locks: {:?}", postgres.get_locks(&dab.1.as_str().unwrap()).await.unwrap());
      println!("-------------------------------------------------");
    }
  }
  println!("{:?}",postgres.databases.lock().unwrap());
  Ok(String::from("Successfully conneced!"))
}

fn main() {
    fast_log::init(Config::new().file("logs/log_info")).unwrap();
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![init_database,repo_test])
    .run(tauri::generate_context!())    
    .expect("error while running tauri application");
}
