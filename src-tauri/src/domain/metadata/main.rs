// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod metadata;
pub mod repository;
pub mod service;
pub mod datb;
pub mod io;
pub mod obfuscation;
use std::collections::HashMap;
use std::default;
use std::{borrow::Borrow, fmt::format, future::IntoFuture, str::FromStr};
use dashmap::DashMap;
use datb::ddl_generate::GenerateDDL;
use datb::dml_generate::{DMLLimit, GenerateDML, Obfuscation};
use fast_log::plugin::console::{self, ConsoleAppender};
use fast_pool::State;
use futures::lock::Mutex;
use metadata::database::Schema;
use rbdc::db::{self, ConnectOptions, Connection, Driver, Placeholder, Row};
use rbdc_mssql::MssqlDriver;
use rbdc_mysql::MysqlDriver;
use rbdc_pg::row::PgRow;
use rbdc_pg:: PgDriver;
use rbdc_sqlite::SqliteDriver;
use rbs::value::map::ValueMap;
use rbs::Value;
use repository::oracle_repository::OracleRepository;
use serde::{Serialize,Deserialize}; 
use strum_macros::{Display, EnumString};
use fast_log::{init, plugin::file, Config};
use log::{info,warn,error};
use tauri::Manager;
use std::sync::Arc;
use crate::repository::postgres_repository::PostgresRepository;
use crate::repository::database_repository::DatabaseRepository;
use crate::repository::mysql_repository::MySqlRepository;
use crate::repository::mssql_repository::MsSqlRepository;
use crate::datb::dml_generate::ObfuscationType;


/* 
#[derive(Default)]
struct AppStateInner{
  repo:DashMap<String,dyn DatabaseRepository>
}

type AppState = Mutex<AppStateInner>;
*/

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
#[allow(unused)]
async fn repo_test(db_connection:DatabaseConnection) -> Result<HashMap<String,Schema>,String> {
  log::info!("Repository");

  match db_connection.driver_type.as_str(){
    "postgresql" => {
      let postgres = Arc::new(PostgresRepository::new(db_connection));
      postgres.init_database().await;
      //postgres.databases.get("products").unwrap().schemas.clone().unwrap().get("public").unwrap().generate_ddls(datb::database_type::DatabaseType::MySql).await;
      println!("tables: {:?}",postgres.databases.get("products").unwrap().schemas.clone().unwrap().get("public").unwrap().tables.clone().unwrap());
      //postgres.databases.get("products").unwrap().schemas.clone().unwrap().get("public").unwrap().tables.clone().unwrap().get("product").unwrap().to_insert(datb::database_type::DatabaseType::MySql,postgres.clone(),"products",Some(DMLLimit{colname : "id".to_string(), limit: 2}),Some(vec!["id".to_string(),"price".to_string()].to_vec()),Some(Obfuscation{type_: ObfuscationType::FIXED,col_name : vec!["name".to_string()]})).await.unwrap(); 
      //postgres.databases.get("products").unwrap().schemas.clone().unwrap().get("public").unwrap().tables.clone().unwrap().get("orders").unwrap().to_insert(datb::database_type::DatabaseType::MsSql,postgres.clone(),"products",None,Some(vec!["id".to_string()]),None).await.unwrap();
      
      let me_in = postgres.databases.get("products").as_ref().unwrap().schemas.clone().unwrap();
      let json_resp = serde_json::to_string(&me_in).unwrap();
      return Ok(postgres.databases.get("products").unwrap().schemas.clone().unwrap());
    },
    "mysql" => {
      let _mysql_ = Arc::new(MySqlRepository::new(db_connection));
      return Ok(HashMap::new());
    },
    "mssql" => {
      let _mssql_ = Arc::new(MsSqlRepository::new(db_connection));
      return Ok(_mssql_.databases.get("products").as_ref().unwrap().schemas.clone().unwrap());
    },
    "sqlite" => {
      let _sqlite_ = Arc::new(MySqlRepository::new(db_connection));
      return Ok(HashMap::new());
    },
    "oracle" => {
      let _oracle_ = Arc::new(OracleRepository::new(db_connection));
      return Ok(HashMap::new());
    }
    _ => {
      return Err(String::from("Not implemented"));
    }
  }

}




fn main() {
    fast_log::init(Config::new().file("logs/log_info")).unwrap();
    //let config = Config::new().console().level(log::LevelFilter::Info).chan_len(Some(100000));
    //fast_log::init(config).unwrap();
    //fast_log::init(Config::new().console().chan_len(Some(100000))).unwrap();
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![repo_test])
    .run(tauri::generate_context!())    
    .expect("error while running tauri application");
}
