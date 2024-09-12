// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use rbdc::{db::{Driver}};
use rbdc_mysql::driver as mysql_driver;
use rbdc_pg::driver as pg_driver;
use rbdc_mssql::driver as ms_driver;
use rbdc_sqlite::driver as sqlite_driver;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
enum DriverType{
  Mysql,
  Pg,
  Mssql,
  Sqlite,
}

fn get_driver(driver_type:DriverType)->Box<dyn Driver>{
  match driver_type{
    DriverType::Mysql => Box::new(mysql_driver::Driver {}),
    DriverType::Pg => Box::new(pg_driver::Driver {}),
    DriverType::Mssql => Box::new(mssql_driver::Driver {}),
    DriverType::Sqlite => Box::new(sqlite_driver::Driver {}),
  }
}



struct ConnectionForm{
  address: String,
  port: u16,
  database: String,
  username: String,
  password: String,
}

#[tauri::command]
fn init_database(connection_form:ConnectionForm){
  let rb = rbdc::RBatis::new();
  let _ = rb.link(get_driver(DriverType::Mysql),format!("{}://{}:{}@{}:{}/{}",connection_form.database,connection_form.username,connection_form.password,connection_form.address,connection_form.port));   
}

fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
