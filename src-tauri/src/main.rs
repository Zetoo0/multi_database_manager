// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


 
pub mod database;
pub mod metadata;

use std::{borrow::Borrow, fmt::format, future::IntoFuture, str::FromStr};
use metadata::{repository::DatabaseRepository, table};
use rbdc::db::{Driver,Connection,ConnectOptions,Row,Placeholder};
use rbdc_mssql::MssqlDriver;
use rbdc_mysql::MysqlDriver;
use rbdc_pg::{types::decode::Decode, value::PgValue, PgDriver};
use rbdc_sqlite::SqliteDriver;
use rbatis::{executor::{Executor, RBatisRef}, Error}; 
use rbs::Value;
use serde::{Serialize,Deserialize}; 
use serde_json::Value as SerdeValue;
use strum_macros::{Display, EnumString, ToString};

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
  let rb = rbatis::RBatis::new();
  let postgres = PostgresRepository::PostgresRepository::new();
  let databases = postgres.get_databases().await.unwrap();
  let mut tables:Value = Value::Null;
  for db_name in databases{
    for dab in db_name.1{
      tables = postgres.get_tables(&dab.1.as_str().unwrap()).await.unwrap();
      println!("Database: {:?}",dab.1.as_str());
      println!("Tables: {:?}",tables);
    }
  }
  Ok(String::from("Successfully conneced!"))
}

fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![init_database,repo_test])
    .run(tauri::generate_context!())    
    .expect("error while running tauri application");
}
