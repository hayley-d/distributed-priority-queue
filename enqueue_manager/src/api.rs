use crate::error::ApiError;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnqueueResponse {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataResponse {
    buffer_size: i64,
    avg_enqueue_latency_ms: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnqueueRequest {
    priority: i32,
    payload: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRequest {
    priority: i32,
    job_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateResponse {
    message: String,
}

#[post("/enqueue", format = "json", data = "<request>")]
pub async fn enqueue(request: Json<EnqueueRequest>) -> Result<Json<EnqueueResponse>, ApiError> {
    todo!()
}

#[get("/metadata")]
pub async fn metadata() -> Result<Json<MetadataResponse>, ApiError> {
    todo!()
}

#[post("/update", format = "json", data = "<request>")]
pub async fn update(request: Json<UpdateRequest>) -> Result<UpdateResponse, ApiError> {
    todo!()
}
