use crate::error::ApiError;
use dotenv::dotenv;
use rocket::fairing::AdHoc;
use rocket::tokio;
use rocket::tokio::sync::Mutex;
use std::env;
use std::sync::Arc;
use tokio_postgres::{Client, NoTls};

pub fn attatch_db() -> AdHoc {
    AdHoc::on_ignite("Attatch DB", |rocket| async {
        match connect_to_db().await {
            Ok(client) => {
                println!("Database Connection Established");
                rocket.manage(Arc::new(Mutex::new(client)))
            }
            Err(e) => {
                eprintln!("Database Connection Error: {:?}", e);
                std::process::exit(1);
            }
        }
    })
}

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
