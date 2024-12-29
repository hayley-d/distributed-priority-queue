use dotenv::dotenv;
use rocket::tokio;
use std::env;
use tokio_postgres::{Client, NoTls};
use tonic::{Code, Status};

pub async fn connect_to_db() -> Result<Client, Status> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| Status::new(Code::Internal, format!("Incorrect Connection String")))?;

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .map_err(|_| {
            Status::new(
                Code::Internal,
                format!("Failed to conntect to the database"),
            )
        })?;

    tokio::spawn(async move { connection.await });

    return Ok(client);
}
