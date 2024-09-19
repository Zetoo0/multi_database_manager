use fast_pool::{Manager, Pool, PooledConnection};
use rbdc_pg::PgDriver;
use tokio_postgres::NoTls;
use std::sync::Arc;
use async_trait::async_trait;


pub struct PgConnectionManager {
    url: String,
}

#[async_trait]
impl Manager for PgConnectionManager{
    type Connection = tokio_postgres::Client;
    type Error = tokio_postgres::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let (client, connection) = tokio_postgres::connect(&self.url, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("PostgreSQL connection error: {:?}", e);
            }
        });
        Ok(client)
    }

    async fn disconnect(&self, _: &Self::Connection) -> Result<(), Self::Error> {
        // Implement if needed
        Ok(())
    }
}