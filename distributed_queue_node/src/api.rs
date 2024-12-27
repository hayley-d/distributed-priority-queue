use std::sync::Arc;

use rocket::serde::json::Json;
use rocket::{get, post};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tokio_postgres::Client;

use crate::error::ApiError;
use crate::min_heap::{HeapNode, MinHeap};

#[derive(Debug, Deserialize, Serialize)]
pub struct DequeueResponse {
    job_id: u64,
    priority: u32,
    payload: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnqueueRequest {
    priority: i32,
    payload: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreationResponse {
    message: String,
}

#[get("/dequeue")]
pub async fn dequeue(
    db: &rocket::State<Arc<Mutex<Client>>>,
) -> Result<Json<DequeueResponse>, ApiError> {
    return Ok(Json(DequeueResponse {
        job_id: 0,
        priority: 0,
        payload: Vec::new(),
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
        payload: Vec::new(),
    }));
}

#[post("/enqueue", format = "json", data = "<request>")]
pub async fn enqueue(
    request: Json<EnqueueRequest>,
    db: &rocket::State<Arc<Mutex<Client>>>,
    heap: &rocket::State<Arc<Mutex<MinHeap>>>,
) -> Result<Json<CreationResponse>, ApiError> {
    let client = db.lock().await;

    let query = client
        .prepare("INSERT INTO jobs (priority, payload) VALUES ($1,$2) RETURNING job_id")
        .await
        .map_err(|_| ApiError::DatabaseError(format!("Error creating query")))?;

    let row = client
        .query_one(&query, &[&request.priority, &request.payload])
        .await
        .map_err(|_| ApiError::DatabaseError(format!("Error creating row")))?;

    let job_id: i64 = row.get(0);

    heap.lock()
        .await
        .insert(request.priority as u32, job_id as u64);

    println!("Inserted job with job_id {} into jobs table", job_id);

    return Ok(Json(CreationResponse {
        message: format!("Job successully added to database"),
    }));
}
