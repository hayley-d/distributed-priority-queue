use std::sync::Arc;

use rocket::get;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tokio_postgres::Client;

use crate::error::ApiError;

#[derive(Debug, Deserialize, Serialize)]
pub struct DequeueResponse {
    job_id: u64,
    priority: u32,
}

#[get("/dequeue")]
pub async fn dequeue(
    db: &rocket::State<Arc<Mutex<Client>>>,
) -> Result<Json<DequeueResponse>, ApiError> {
    return Ok(Json(DequeueResponse {
        job_id: 0,
        priority: 0,
    }));
}

#[get("/dequeue/<amount>")]
pub async fn dequeue_amount(
    amount: String,
    db: &rocket::State<Arc<Mutex<Client>>>,
) -> Result<Json<DequeueResponse>, ApiError> {
    return Ok(Json(DequeueResponse {
        job_id: 0,
        priority: 0,
    }));
}
