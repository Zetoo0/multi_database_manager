use crate::metadata::table;
use crate::metadata::{ table:: Table, trigger, view::View};
    use crate::repository::database_repository::DatabaseRepository;
    use dashmap::mapref::one::Ref;
    use rbdc_sqlite::*;
    use rbs::Value;
    use std::collections::HashMap;
    use std::sync::Arc;
    use log::{error, info, warn};
    use crate::metadata::database::Database;
    use dashmap::DashMap;


#[derive(Debug,Clone)]
pub struct SqLiteRepository{
    rb_map:DashMap<String,Arc<rbatis::RBatis>>,
    base_path:String,
    pub databases:DashMap<String,Database>,
}

impl SqLiteRepository {
    pub fn new(path:&str)->Self{
        let rb_map = DashMap::new();
        let databases = DashMap::new();
        let base_path = String::from(format!("sqlite://{}",path));
        return SqLiteRepository{ rb_map, base_path, databases};
    }

        ///Add the database to the pool if not exists
        ///It's create a new rbatis, initialize it add add to the pool
    async fn connect(&self,name:&str,path:&str) -> Result<(), Box<dyn std::error::Error>> {
        if !self.rb_map.contains_key(name){
            log::info!("new pool adding... database: {:?}",name);
            let rb = Arc::new(rbatis::RBatis::new());
            match rb.init(SqliteDriver {}, path) {
                Ok(_) => log::info!("Connection to {} successful", name),
                Err(e) => {
                    log::error!("Failed to initialize rbatis for {}: {:?}", name, e);
                    return Err(Box::new(e)); // Return the error early
                }
            }
            self.rb_map.insert(name.to_string(),rb);
            let db_node = Database{
                name : name.to_string(),
                tables : Some(HashMap::new()),
                functions : None,                       
                procedures : Some(HashMap::new()),
                views : Some(HashMap::new()),
                constraints : Some(HashMap::new()),
                foreign_data_wrappers : None,
                locks : None,
                types : None,
                triggers : Some(HashMap::new()),
                aggregates : None,
                materalized_views : None,
                catalogs : None,
                sequences : None,
            };
            self.databases.insert(name.to_string(), db_node);
            log::info!("Database node created for {}", name);
        }
        Ok(())
        }

        ///Get the database by its name
        async fn get_database_(&self,db_name:&str)->std::option::Option<dashmap::mapref::one::Ref<'_, std::string::String, Database, >>{
            self.databases.get(db_name)
        }

        ///connect to rbatis if it isnt cached
        async fn rbatis_connect(&self,db_name:&str)->Result<Option<Ref<'_,String,Arc<rbatis::RBatis>>>,rbdc::Error>{
            let cached_rb = self.rb_map.get(db_name);
            if cached_rb.is_some(){
                log::info!("rb cached");
                return Ok(cached_rb);
            }
            log::info!("rb isnt cached");          
            if let Err(err) = self.connect(db_name, &self.base_path).await {
                log::error!("Connection failed for database {}: {:?}", db_name, err);
                return Err(rbdc::Error::from("Failed to connect to database"));
            }
            match self.rb_map.get(db_name) {
                Some(rb) => {
                    log::info!("Connection successfull, rb cached");
                    Ok(Some(rb))
                }
                None => {
                    log::error!("Connection failed to cache rbatis");
                    Err(rbdc::Error::from("Database not found after connection attempt"))
                }
            }
            //Ok(rb)
        }

}

impl DatabaseRepository for SqLiteRepository{
    async fn get_databases(&self)-> Result<Value,rbdc::Error> {
        log::info!("Sqlite: Get databases");
        let rb = self.rbatis_connect("postgres").await?.unwrap();
        let _sql = "PRAGMA database_list;";
        println!("rb: {:?}",rb);
        let result = rb.query(_sql,vec![]).await.unwrap();
        //iterate through databases and insert into the pool and the database map(db.1 = database name)
        if let Some(databases) = result.as_array(){
            for db_val in databases{
                for db in db_val{
                    if !self.rb_map.contains_key(db.1.as_str().unwrap()){
                        let rb = Arc::new(rbatis::RBatis::new());

                        self.rb_map.insert(db.1.to_string(),rb);

                        let db_node = Database{
                            name : db.1.to_string(),
                            tables : Some(HashMap::new()),
                            functions : Some(HashMap::new()),                        
                            procedures : Some(HashMap::new()),
                            views : Some(HashMap::new()),
                            constraints : Some(HashMap::new()),
                            foreign_data_wrappers : Some(HashMap::new()),
                            locks : Some(HashMap::new()),
                            types : Some(HashMap::new()),
                            triggers : Some(HashMap::new()),
                            aggregates : Some(HashMap::new()),
                            materalized_views : Some(HashMap::new()),
                            catalogs : Some(HashMap::new()),
                            sequences : Some(HashMap::new()),
                        };
                        self.databases.insert(db.1.to_string(), db_node);
                    }
                }
            }
        }
        Ok(result)
    }

    async fn get_tables(&self, db_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();
        let _sql = "SELECT name
                        FROM sqlite_master
                        WHERE type = 'table';";
        let result = rb.query(_sql,vec![]).await.unwrap();
        if result.is_empty(){
            return Ok(Value::Null);
        }
        log::info!("Get tables...");
        if let Some(tables) = result.as_array(){
            if let Some(mut node) = self.databases.get_mut(db_name){
                let table_map = node.value_mut().tables.get_or_insert_with(HashMap::new);
                for table in tables{
                    if let Value::Map(tablemap) = table{
                        let table_name = tablemap.0.get(&Value::String("name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                    
                        let tb_node = Table{
                            name : table_name.to_string(), 
                            columns : Some(HashMap::new()),
                            constraints: Some(HashMap::new()),
                            indexes: Some(HashMap::new()),
                            triggers: Some(HashMap::new()),
                            rules: Some(HashMap::new()),
                            rls_policies: Some(HashMap::new()),
                        };
                        table_map.insert(table_name.to_string(), tb_node);
                    }
                }
            }else{
                log::info!("No nodes")
            }
        }else{
            log::info!("result is empty");
        }

        println!("BASIC TABLE RESULT STRUCT: {:?}", result);
        
        Ok(result)
    }

    ///Get all columns in the table
    async fn get_columns(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "PRAGMA table_info(?);";
        
        let result = rb.query(_sql, vec![Value::String(table_name.to_string())]).await.unwrap();
    
        if let Some(cols) = result.as_array(){
            if let Some(mut node) = self.databases.get_mut(db_name){
                if let Some(table_map) = node.value_mut().tables.clone().unwrap().get_mut(db_name){
                    let columns_map = table_map.columns.get_or_insert_with(HashMap::new);
                    for col in cols{
                        if let Value::Map(colmap) = col{
                            let col_name = colmap.0.get(&Value::String("name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let data_type = colmap.0.get(&Value::String("type".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let is_nullable = colmap.0.get(&Value::String("notnull".to_string())).and_then(|v| v.as_bool()).unwrap_or_default();
                            let column_default = colmap.0.get(&Value::String("dflt_value".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            
                            let _col_node = crate::metadata::column::Column{
                                name : String::from(col_name),//col_name.1.to_string(),
                                data_type: Some(String::from(data_type)),     
                                is_nullable: Some(is_nullable),
                                default_value: Some(String::from(column_default)),
                            };
                            
                            columns_map.insert(col_name.to_string(), _col_node);
                        }
                    }
                }
            }else{
                log::info!("Node is not OK");
            }
        }else{
            log::info!("No columns");
        }

        Ok(result)
    }

    async fn get_views(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();
        
        let _sql = "SELECT name,sql
                        FROM sqlite_master
                        WHERE type = 'view';";
        let result = rb.query(_sql, vec![]).await.unwrap();
        
        if let Some(views) = result.as_array(){
           // let mut db_struct = self.databases.lock().unwrap();
            if let Some(mut node) = self.databases.get_mut(db_name){
                let view_map = node.value_mut().views.get_or_insert_with(HashMap::new);
                for view in views{
                    if let Value::Map(viewmap) = view{
                        let view_name = viewmap.0.get(&Value::String("name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                        let view_definition = viewmap.0.get(&Value::String("sql".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                        let view_node = View{
                            name : view_name.to_string(),
                            definition : view_definition.to_string(),
                        };
                        view_map.insert(view_name.to_string(), view_node);
                    }
                }
            }else{
                log::info!("Node is not OK");
            }
        }else{
            log::info!("No views");
        }

        Ok(result)
    }

    async fn get_stored_procedures(&self,db_name:&str)-> Result<Value,rbdc::Error> {
       Ok(Value::Null)
    }

    async fn get_functions(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)
    }

    async fn get_trigger_functions(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql =  "SELECT name 
                        FROM sqlite_master 
                        WHERE type = 'trigger';";
        let result = rb.query(_sql, vec![]).await.unwrap();
        if let Some(triggers) = result.as_array(){
            //let mut db_struct = self.databases.lock().unwrap();
            if let Some(mut node) = self.databases.get_mut(db_name){
                let trigger_map = node.value_mut().triggers.get_or_insert_with(HashMap::new);
                for trigger in triggers{
                    if let Value::Map(triggermap) = trigger{
                        let trigger_name = triggermap.0.get(&Value::String("name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                        let trigger_node = trigger::Trigger{
                            name : trigger_name.to_string(),
                        };
                        trigger_map.insert(trigger_name.to_string(), trigger_node);
                    }
                }
            }else{
                log::info!("Node is not OK");
            }
        }else{
            log::info!("No triggers");
        }
        Ok(result)
    }

    async fn get_event_triggers(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)

    }

    async fn get_aggregates(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)
    }

    async fn get_materalized_views(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)
    }

    ///Get the types from the database if exists
    async fn get_types(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)
    }

    async fn get_languages(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)

    }

    async fn get_catalogs(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)
    }

    async fn get_foreign_data_wrappers(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)
    }

    //TODO SELECT table_name FROM information_schema.tables WHERE table_schema = '?'; catalogobjects(?)
    async fn get_schemas(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)
    }

    async fn get_indexes(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT name AS index_name, sql AS definition
                        FROM sqlite_master 
                        WHERE type = 'index';";
        let result = rb.query(_sql, vec![]).await.unwrap();
        
        if let Some(indexes) = result.as_array(){
            if let Some(mut node) = self.databases.get_mut(db_name){
                if let Some(table_map) = node.value_mut().tables.clone().unwrap().get_mut(db_name){
                    let index_map = table_map.indexes.get_or_insert_with(HashMap::new);
                    for index in indexes{
                        if let Value::Map(indexmap) = index{
                            let index_name = indexmap.0.get(&Value::String("index_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let index_definition = indexmap.0.get(&Value::String("definition".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            
                            let index_node = crate::metadata::index::Index{
                                name : String::from(index_name),
                                definition: Some(String::from(index_definition)),     
                            };
                            index_map.insert(index_name.to_string(), index_node);
                        }
                    }
                }
            }else{
                log::info!("Node is not OK");
            }
        }else{
            log::info!("No indexes");
        }

        Ok(result)
    }

    async fn get_constraints(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)

    }

    async fn get_sequences(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)

    }

    async fn get_roles_and_users(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)

    }

    async fn get_table_statistics(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)


    }

    async fn get_active_sessions(&self)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)


    }

    async fn get_locks(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)


    }

    async fn get_partitions(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)


    }

    async fn get_user_privileges(&self,db_name:&str,user_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)

    }

    async fn get_database_settings(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)

    }

    async fn get_foreign_key_relationships(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)

    }

    async fn get_triggers_associated_with_table(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)

    }

    async fn get_default_columns_value(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)


    }

    async fn get_rls_policies(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)


    }

    async fn get_rules(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)


    }
}