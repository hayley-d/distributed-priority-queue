use crate::error::ApiError;
use dotenv::dotenv;
use rocket::tokio;
use std::env;
use tokio_postgres::{Client, NoTls};

pub async fn connect_to_db() -> Result<Client, ApiError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| ApiError::DatabaseError(format!("Incorrect Connection String")))?;

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    tokio::spawn(async move { connection.await });

    return Ok(client);
}
