use crate::metadata::{function::Function, materalized_view::MateralizedView, procedure::Procedure, table:: Table, trigger, view::View};
    use crate::metadata::rls_policy::RlsPolicy;
    use crate::metadata::sequence::Sequence;
    use crate::metadata::aggregate::Aggregate;
    use crate::metadata::catalog::Catalog;
    use crate::repository::database_repository::DatabaseRepository;
    use dashmap::mapref::one::Ref;
    use driver::OracleDriver;
    use rbdc_oracle::*;
    use std::collections::HashMap;
    use rbs::Value;
    use std::sync::Arc;
    use log::{error, info, warn};
    use crate::metadata::database::Database;
    use dashmap::DashMap;
    use crate::DatabaseConnection;

#[derive(Clone)]
pub struct OracleRepository{
    rb_map:DashMap<String,Arc<rbatis::RBatis>>,
    base_url:String,
    pub databases:DashMap<String,Database>,
    connection_info:DatabaseConnection,
}

impl OracleRepository{
    pub fn new(connection_info:DatabaseConnection) -> Self{
        let rb_map = DashMap::new();
        let databases = DashMap::new();
        let base_url = String::from(format!("{}://{}:{}@{}:{}/oracle",connection_info.driver_type,
                                        connection_info.username,connection_info.password,
                                        connection_info.server,connection_info.port));
        return OracleRepository{ rb_map, base_url, databases, connection_info};
    }

    ///Add the database to the pool if not exists
    ///It's create a new rbatis, initialize it add add to the pool
    async fn connect(&self,db_name:&str,url:&str) -> Result<(), Box<dyn std::error::Error>> {
        if !self.rb_map.contains_key(db_name){
            log::info!("new pool adding... database: {:?}",db_name);
            let rb = Arc::new(rbatis::RBatis::new());
            match rb.init(OracleDriver {}, url) {
                Ok(_) => log::info!("Connection to {} successful", db_name),
                Err(e) => {
                    log::error!("Failed to initialize rbatis for {}: {:?}", db_name, e);
                    return Err(Box::new(e)); // Return the error early
                }
            }
            self.rb_map.insert(db_name.to_string(),rb);
            let db_node = Database{
                name : db_name.to_string(),
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
            self.databases.insert(db_name.to_string(), db_node);
            log::info!("Database node created for {}", db_name);
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
        let url = String::from(format!("{}://{}:{}@{}:{}/{}",
                        self.connection_info.driver_type,self.connection_info.username,
                        self.connection_info.password,self.connection_info.server,
                        self.connection_info.port,db_name));
        if let Err(err) = self.connect(db_name, &url).await {
            println!("Connection failed for database {}: {:?}", db_name, err);
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
        //SELECT username FROM all_users;

impl DatabaseRepository for OracleRepository {
    async fn get_databases(&self)-> Result<Value,rbdc::Error> {
        log::info!("Oraclerepository: Get databases");
        let rb = self.rbatis_connect("postgres").await?.unwrap();
        let _sql = "SELECT username FROM all_users;";
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

    ///Get all tables in the database
    async fn get_tables(&self, db_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();
        let _sql = "SELECT  table_name
                        FROM all_tables
                        WHERE owner = '?';";
        let result = rb.query(_sql,vec![Value::String(String::from(db_name))]).await.unwrap();
        if result.is_empty(){
            return Ok(Value::Null);
        }
        log::info!("Get tables...");
        if let Some(tables) = result.as_array(){
            if let Some(mut node) = self.databases.get_mut(db_name){
                let table_map = node.value_mut().tables.get_or_insert_with(HashMap::new);
                for table in tables{
                    if let Value::Map(tablemap) = table{
                        let table_name = tablemap.0.get(&Value::String("table_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                    
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

        let _sql = "SELECT 
                        column_name,
                        data_type,
                        data_length,
                        nullable
                    FROM all_tab_columns
                    WHERE table_name = '?'
                    AND owner = '?';";
        
        let result = rb.query(_sql, vec![Value::String(table_name.to_string()),Value::String(db_name.to_string())]).await.unwrap();
        //let result_2:Vec<ValueMap> = rb.query_decode(_sql, vec![Value::String(table_name.to_string())]).await.unwrap();
        //println!("DECODE RESULTS");
    
        if let Some(cols) = result.as_array(){
            if let Some(mut node) = self.databases.get_mut(db_name){
                if let Some(table_map) = node.value_mut().tables.clone().unwrap().get_mut(db_name){
                    let columns_map = table_map.columns.get_or_insert_with(HashMap::new);
                    for col in cols{
                        if let Value::Map(colmap) = col{
                            let col_name = colmap.0.get(&Value::String("column_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let data_type = colmap.0.get(&Value::String("data_type".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let is_nullable = colmap.0.get(&Value::String("is_nullable".to_string())).and_then(|v| v.as_bool()).unwrap_or_default();
                            //let column_default = colmap.0.get(&Value::String("column_default".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            
                            let _col_node = crate::metadata::column::Column{
                                name : String::from(col_name),//col_name.1.to_string(),
                                data_type: Some(String::from(data_type)),     
                                is_nullable: Some(is_nullable),
                                default_value: None,
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
        
        let _sql = "SELECT 
                        view_name
                        FROM all_views
                        WHERE owner = '?';";
        let result = rb.query(_sql, vec![Value::String(db_name.to_string())]).await.unwrap();
        
        if let Some(views) = result.as_array(){
           // let mut db_struct = self.databases.lock().unwrap();
            if let Some(mut node) = self.databases.get_mut(db_name){
                let view_map = node.value_mut().views.get_or_insert_with(HashMap::new);
                for view in views{
                    if let Value::Map(viewmap) = view{
                        let view_name = viewmap.0.get(&Value::String("table_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                        let view_definition = viewmap.0.get(&Value::String("view_definition".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                        let view_node = View{
                            name : view_name.to_string(),
                            definition : "View ".to_string(),
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
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT object_name
                            FROM all_procedures
                            WHERE object_type = 'PROCEDURE'
                            AND owner = '?';";
        let result = rb.query(_sql, vec![Value::String(String::from(db_name))]).await.unwrap();

        if let Some(stored_procedures) = result.as_array(){
            //let mut db_struct = self.databases.lock().unwrap();
            if let Some(mut node) = self.databases.get_mut(db_name){
                let stored_procedure_map = node.value_mut().procedures.get_or_insert_with(HashMap::new);
                for stored_procedure in stored_procedures{
                    if let Value::Map(stored_proceduremap) = stored_procedure{
                        let stored_procedure_name = stored_proceduremap.0.get(&Value::String("routine_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                        let stored_procedure_node = Procedure{
                            name : stored_procedure_name.to_string(),
                            definition : "Stored Procedure ".to_string(),
                        };
                        stored_procedure_map.insert(stored_procedure_name.to_string(), stored_procedure_node);
                    }
                }
            }else{
                log::info!("Node is not OK");
            }
        }else{
            log::info!("No stored procedures");
        }
        Ok(result)
    }

    async fn get_functions(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                        object_name,
                        object_type
                    FROM all_procedures
                    WHERE object_type = 'FUNCTION'
                    AND owner = '?';";
        let result = rb.query(_sql, vec![Value::String(String::from(db_name))]).await.unwrap();

        if let Some(functions) = result.as_array(){
            //let mut db_struct = self.databases.lock().unwrap();
            if let Some(mut node) = self.databases.get_mut(db_name){
                let function_map = node.value_mut().functions.get_or_insert_with(HashMap::new);
                for function in functions{
                    if let Value::Map(functionmap) = function{
                        let function_name = functionmap.0.get(&Value::String("object_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                        let function_schema = functionmap.0.get(&Value::String("object_type".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                        let function_node = Function{
                            name : function_name.to_string(),
                            definition : function_schema.to_string(),
                        };
                    }
                }
            }else{
                log::info!("Node is not OK");
            }
        }else{
            log::info!("No functions");
        }
        Ok(result)
    }

    async fn get_trigger_functions(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql =  "SELECT 
                            trigger_name,
                        FROM all_triggers
                        WHERE owner = '?';";
        let result = rb.query(_sql, vec![Value::String(String::from(db_name))]).await.unwrap();
        if let Some(triggers) = result.as_array(){
            //let mut db_struct = self.databases.lock().unwrap();
            if let Some(mut node) = self.databases.get_mut(db_name){
                let trigger_map = node.value_mut().triggers.get_or_insert_with(HashMap::new);
                for trigger in triggers{
                    if let Value::Map(triggermap) = trigger{
                        let trigger_name = triggermap.0.get(&Value::String("trigger_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
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
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT evtname 
        FROM pg_event_trigger;";
        let result = rb.query(_sql, vec![]).await.unwrap();

        Ok(result)

    }

    async fn get_aggregates(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT proname 
        FROM pg_proc 
        WHERE prokind='a';";
        let result = rb.query(_sql, vec![]).await.unwrap();
        if let Some(aggregates) = result.as_array(){
           // let mut db_struct = self.databases.lock().unwrap();
            if let Some(mut node) = self.databases.get_mut(db_name){
                let aggregate_map = node.value_mut().aggregates.get_or_insert_with(HashMap::new);
                for aggregate in aggregates{
                    if let Value::Map(aggregatemap) = aggregate{
                        let aggregate_name = aggregatemap.0.get(&Value::String("proname".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                        let aggregate_node = Aggregate{
                            name : aggregate_name.to_string(),
                        };
                        aggregate_map.insert(aggregate_name.to_string(), aggregate_node);
                    }
                }
            }
        }
        Ok(result)
    }

    async fn get_materalized_views(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();
        let _sql = "SELECT matviewname,definition
        FROM pg_matviews;";
        let result = rb.query(_sql, vec![]).await.unwrap();

        if let Some(matview) = result.as_array(){
            if let Some(mut node) = self.databases.get_mut(db_name){
                let matview_map = node.value_mut().materalized_views.get_or_insert_with(HashMap::new);
                for matview in matview{
                    if let Value::Map(matviewmap) = matview{
                        let matview_name = matviewmap.0.get(&Value::String("matviewname".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                        let matview_def = matviewmap.0.get(&Value::String("definition".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                        let matview_node = MateralizedView{
                            name : matview_name.to_string(),
                            definition : matview_def.to_string(),
                        };
                        matview_map.insert(matview_name.to_string(), matview_node);
                    }
                }
            }else{
                log::info!("Node is not OK");
            }
        }else{
            log::info!("No materalized views");
        }
        Ok(result)
    }

    ///Get the types from the database if exists
    async fn get_types(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT typname 
        FROM pg_type 
        WHERE typtype = 'b';";
        let result = rb.query(_sql, vec![]).await.unwrap();
        Ok(result)
    }

    async fn get_languages(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        let url = "postgresql://mzeteny:zetou123@localhost:5432/".to_string()+db_name;
        let _ = self.connect(db_name,url.as_str()).await;
        let rb = match self.rb_map.get(db_name){
            Some(rb) => rb,
            None => return Err(rbdc::Error::from("database not found")),
        };
        let _sql ="SELECT lanname 
        FROM pg_language;";
        let result = rb.query(_sql, vec![]).await.unwrap();
        
        Ok(result)

    }

    async fn get_catalogs(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT nspname AS catalog_name
                        FROM pg_namespace
                        WHERE nspname IN ('pg_catalog', 'information_schema')
                        ORDER BY nspname;";
        let result = rb.query(_sql, vec![]).await.unwrap();

        if let Some(catalogs) = result.as_array(){
           // let mut db_struct = self.databases.lock().unwrap();
            if let Some(mut node) = self.databases.get_mut(db_name){
                let catalog_map = node.value_mut().catalogs.get_or_insert_with(HashMap::new);
                for catalog in catalogs{
                    if let Value::Map(catalogmap) = catalog{
                        let catalog_name = catalogmap.0.get(&Value::String("catalog_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                        
                        let catalog_node = Catalog{
                            name : catalog_name.to_string(),
                        };
                        
                        catalog_map.insert(catalog_name.to_string(), catalog_node);
                    }
                }
            }else{
                log::info!("Node is not OK");
            }
        }else{
            log::info!("No catalogs");
        }
        Ok(result)
    }

    async fn get_foreign_data_wrappers(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT fdwname 
        FROM pg_foreign_data_wrapper;";
        let result = rb.query(_sql, vec![]).await.unwrap();

        if let Some(foreign_data_wrappers) = result.as_array(){
            //let mut db_struct = self.databases.lock().unwrap();
            if let Some(mut node) = self.databases.get_mut(db_name){
                let foreign_data_wrapper_map = node.value_mut().foreign_data_wrappers.get_or_insert_with(HashMap::new);
                for foreign_data_wrapper in foreign_data_wrappers{
                    if let Value::Map(fdw_map) = foreign_data_wrapper{
                        let foreign_data_wrapper_name = fdw_map.0.get(&Value::String("fdwname".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                        
                        let foreign_data_wrapper_node = crate::metadata::foreign_data_wrapper::ForeignDataWrapper{
                            name : foreign_data_wrapper_name.to_string(),
                        };
                        
                        foreign_data_wrapper_map.insert(foreign_data_wrapper_name.to_string(), foreign_data_wrapper_node);
                    }
                }
            }else{
                log::info!("Node is not OK");
            }
        }else{
            log::info!("No foreign data wrappers");
        }
        Ok(result)
    }

    //TODO SELECT table_name FROM information_schema.tables WHERE table_schema = '?'; catalogobjects(?)
    async fn get_schemas(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT schema_name 
        FROM information_schema.schemata
        WHERE schema_name='public';";
        let result = rb.query(_sql, vec![]).await.unwrap();
        Ok(result)
    }

    async fn get_indexes(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                        index_name,
                    FROM all_indexes
                    WHERE table_owner = '?';";
        let result = rb.query(_sql, vec![Value::String(String::from(db_name))]).await.unwrap();
        
        if let Some(indexes) = result.as_array(){
            if let Some(mut node) = self.databases.get_mut(db_name){
                if let Some(table_map) = node.value_mut().tables.clone().unwrap().get_mut(db_name){
                    let index_map = table_map.indexes.get_or_insert_with(HashMap::new);
                    for index in indexes{
                        if let Value::Map(indexmap) = index{
                            let index_name = indexmap.0.get(&Value::String("index_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            
                            let index_node = crate::metadata::index::Index{
                                name : String::from(index_name),
                                definition: None,     
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
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                        constraint_name,
                        constraint_type,
                    FROM all_constraints
                    WHERE owner = '?';";
        let result = rb.query(_sql, vec![Value::String(String::from(db_name))]).await.unwrap();
        if let Some(constraints) = result.as_array(){
            if let Some(mut node) = self.databases.get_mut(db_name){
                if let Some(table_map) = node.value_mut().tables.clone().unwrap().get_mut(table_name){
                    let constraint_map = table_map.constraints.get_or_insert_with(HashMap::new);
                    for constraint in constraints{
                        if let Value::Map(colmap) = constraint{
                            let constraint_name = colmap.0.get(&Value::String("constraint_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let constraint_type = colmap.0.get(&Value::String("constraint_type".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            
                            let constraint_node = crate::metadata::constraint::Constraint{
                                name: String::from(constraint_name),
                                c_type: String::from(constraint_type)
                            };
                            constraint_map.insert(constraint_name.to_string(), constraint_node);
                        }
                    }
                }
                
            }else{
                log::info!("Node is not OK");
            }
        }else{
            log::info!("No constraints");
        }
        Ok(result)

    }

    async fn get_sequences(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                        sequence_name
                    FROM all_sequences
                    WHERE sequence_owner = 'YOUR_SCHEMA';";
        let result = rb.query(_sql, vec![Value::String(String::from(db_name))]).await.unwrap();

        if let Some(sequences) = result.as_array(){
            //let mut db_struct = self.databases.lock().unwrap();
            if let Some(mut node) = self.databases.get_mut(db_name){
                let sequence_map = node.value_mut().sequences.get_or_insert_with(HashMap::new);
                for seq in sequences{
                    if let Value::Map(sequencemap) = seq{
                        let sequence_name = sequencemap.0.get(&Value::String("sequence_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                        let sequence_node = Sequence{
                            name : sequence_name.to_string(),
                        };
                        sequence_map.insert(sequence_name.to_string(), sequence_node);
                    }
                }
            }else{
                log::info!("Node is not OK");
            }
        }else{
            log::info!("No sequences");
        }

        Ok(result)

    }

    async fn get_roles_and_users(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                        username
                    FROM all_users;";
        let result = rb.query(_sql, vec![]).await.unwrap();
        Ok(result)

    }

    async fn get_table_statistics(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                    table_name,
                    num_rows,
                    blocks,
                    empty_blocks,
                    avg_row_len
                FROM all_tables
                WHERE owner = 'YOUR_SCHEMA';";
        let result = rb.query(_sql, vec![Value::String(String::from(db_name))]).await.unwrap();
        Ok(result)

    }

    async fn get_active_sessions(&self)-> Result<Value,rbdc::Error> {
        self.connect("postgres",self.base_url.as_str()).await;
        let rb = match self.rb_map.get("postgres"){
            Some(rb) => rb,
            None => return Err(rbdc::Error::from("database not found")),
        };
        let _sql = "SELECT 
                        sid,
                        serial#,
                        username,
                        status,
                        machine
                    FROM v$session;";
        let result = rb.query(_sql, vec![]).await.unwrap();
        Ok(result)

    }

    async fn get_locks(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();
        let _sql = "SELECT 
        sid,
        serial#,
        type,
        mode_held,
        mode_requested
    FROM v$lock;";
        let result = rb.query(_sql, vec![]).await.unwrap();
        Ok(result)

    }

    async fn get_partitions(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                        partition_name,
                    FROM all_tab_partitions
                    WHERE table_owner = '?';";
        let result = rb.query(_sql, vec![Value::String(table_name.to_string())]).await.unwrap();
        Ok(result)

    }

    async fn get_user_privileges(&self,db_name:&str,user_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                grantee,
                privilege,
                table_name
            FROM all_tab_privs
            WHERE owner = 'YOUR_SCHEMA';";
        let result = rb.query(_sql, vec![Value::String(db_name.to_string())]).await.unwrap();
        Ok(result)
    }

    async fn get_database_settings(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT 
                        parameter,
                        value
                    FROM v$parameter;";
        let result = rb.query(_sql, vec![]).await.unwrap();
        Ok(result)
    }

    async fn get_foreign_key_relationships(&self,db_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT conname AS constraint_name, conrelid::regclass AS table_name,
    a.attname AS column_name, confrelid::regclass AS foreign_table_name,
    af.attname AS foreign_column_name
        FROM   pg_constraint
        JOIN   pg_attribute a ON a.attnum = ANY(conkey) AND a.attrelid = conrelid
        JOIN   pg_attribute af ON af.attnum = ANY(confkey) AND af.attrelid = confrelid
        WHERE  contype = 'f';";
        let result = rb.query(_sql, vec![]).await.unwrap();
        Ok(result)
    }

    async fn get_triggers_associated_with_table(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT tgname
        FROM pg_trigger
        WHERE tgrelid = ?::regclass;";
        let result = rb.query(_sql, vec![Value::String(table_name.to_string())]).await.unwrap();
        Ok(result)
    }

    async fn get_default_columns_value(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT column_name, column_default
        FROM information_schema.columns
        WHERE table_name = ?;";
        let result = rb.query(_sql, vec![Value::String(table_name.to_string())]).await.unwrap();
        Ok(result)

    }

    async fn get_rls_policies(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
        let rb = self.rbatis_connect(db_name).await?.unwrap();

        let _sql = "SELECT policy_name FROM DBA_POLICIES WHERE OBJECT_NAME = 'your_table_name';";
        let result = rb.query(_sql, vec![Value::String(table_name.to_string())]).await.unwrap();

        if let Some(policies) = result.as_array(){
            if let Some(mut node) = self.databases.get_mut(db_name){
                if let Some(table_map) = node.value_mut().tables.clone().unwrap().get_mut(table_name){
                    let policy_map = table_map.rls_policies.get_or_insert_with(HashMap::new);
                    for policy in policies{
                        if let Value::Map(polmap) = policy{
                            let policy_name = polmap.0.get(&Value::String("policy_name".to_string())).and_then(|v| v.as_str()).unwrap_or_default();
                            let policy_node = RlsPolicy{
                                name : policy_name.to_string(),
                                command : polmap.0.get(&Value::String("command".to_string())).and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                            };
                            policy_map.insert(policy_name.to_string(), policy_node);
                        }
                    }
                }
            }else{
                log::info!("Node is not OK");
            }
        }else{
            log::info!("No policies");
        }
        Ok(result)

    }

    async fn get_rules(&self,db_name:&str,table_name:&str)-> Result<Value,rbdc::Error> {
        Ok(Value::Null)

    }
}