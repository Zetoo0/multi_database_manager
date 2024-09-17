// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


 
use std::{fmt::format, future::IntoFuture, str::FromStr};
use rbdc::db::{Driver,Connection,ConnectOptions,Row,Placeholder};
use rbdc_mssql::MssqlDriver;
use rbdc_mysql::MysqlDriver;
use rbdc_pg::{PgDriver};
use rbdc_sqlite::SqliteDriver;
use rbatis::{executor::{Executor, RBatisRef}, Error}; 
use serde::{Serialize,Deserialize}; 
use serde_json::Value;
use strum_macros::{Display, EnumString, ToString};

struct Table{

}

trait Lister {
  fn list_tables(&self)->Vec<Table>{
    vec![]
  }
  fn list_views(&self);
  fn list_stored_procedures(&self);
  fn list_columns(&self,table:&str);
}

/* 
impl Lister for rbdc_pg::PgDriver{
  fn list_tables(&self)->Vec<Table> {
      
  }

  fn list_columns(&self,table:&str) {
      
  }

  fn list_stored_procedures(&self) {

  }

  fn list_views(&self) {
      
  }
}*/


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

#[derive(Serialize, Deserialize)]
struct DatabaseConnection{
  port: String,
  server: String,
  username: String,
  password: String,
  driver_type: String,
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
      let query_res = rb.query("SELECT datname
                                          FROM pg_database WHERE datistemplate=false;",vec![]);
      println!("{:?}",query_res.await.map(|rows| rows.into_iter().map(|row| row.1)));
      Ok(String::from("Successfully conneced!"))
    },
    Err(err) => Err(err.to_string())
  }
}

fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![init_database])
    .run(tauri::generate_context!())    
    .expect("error while running tauri application");

}
