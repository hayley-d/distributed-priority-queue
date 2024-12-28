use crate::error::ApiError;
use crate::min_heap::{HeapNode, MinHeap};
use log::error;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

/// Contains the API routes for the [distributed_queue_node] in the [distributed priority queue].
///
/// The node contains the following API routes:
///
/// GET /dequeue
/// Used to pop the job with the highest priority from the priority queue.
///
/// Example Response
/// ```json
/// {
///     "job_id" : 123,
///     "priority" : 1,
///     "payload" : [1,1,1,1]
/// }
/// ```
/// GET /dequeue/<amount>
/// Used to pop <amount> jobs from the priority queue.
///
/// Example Response
/// ```json
/// {
///     [
///         {"job_id":123,"priority":1,"payload":[]},
///         {"job_id":124,"priority":2,"payload":[]}   
///     ]
/// }
/// ```
/// POST /enqueue
/// Enqueues a job to the priority queue.
///
/// Example Request:
/// ```json
/// {
///     "priority" : 1,
///     "payload" : [],
/// }
/// ```
///

/// DequeueResponse represents the response sent by the node when the /dequeue route is used.
#[derive(Debug, Deserialize, Serialize)]
pub struct DequeueResponse {
    job_id: i64,
    priority: i32,
    payload: Vec<u8>,
}

/// BatchDequeueResponse represents the response sent by the node when the /dequeue/<amount> route
/// is used.
#[derive(Debug, Deserialize, Serialize)]
pub struct BatchDequeueResponse {
    jobs: Vec<DequeueResponse>,
}

/// EnqueueRequest represents the request expected when the /enqueue route is used.
#[derive(Debug, Deserialize, Serialize)]
pub struct EnqueueRequest {
    priority: i32,
    payload: Vec<u8>,
}

/// CreationResponse is the response sent by the node when a job is successfully added into the
/// priortiy queue.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreationResponse {
    message: String,
    job_id: u64,
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

#[get("/dequeue")]
pub async fn dequeue(
    db: &rocket::State<Arc<Mutex<Client>>>,
    heap: &rocket::State<Arc<Mutex<MinHeap>>>,
    clock: &rocket::State<Arc<Mutex<u64>>>,
) -> Result<Json<DequeueResponse>, ApiError> {
    let client = db.lock().await;

    let node: HeapNode = match heap.lock().await.get_top() {
        Some(n) => n,
        None => {
            error!("Error: Attempt to pull from empty heap");
            return Err(ApiError::EmptyHeapError);
        }
    };

    // Increment logical time
    *clock.lock().await += 1;

    let query = client
        .prepare("SELECT * FROM jobs WHERE job_id = $1")
        .await
        .map_err(|_| {
            error!("Error: Failed to create SELECT query");
            ApiError::DatabaseError(format!("Error creating query"))
        })?;

    let row = client
        .query_one(&query, &[&(node.job_id as i64)])
        .await
        .map_err(|_| {
            error!("Error: Attempt to SELECT from database failed");
            ApiError::DatabaseError(format!("Error database SELECT query failed."))
        })?;

    return Ok(Json(DequeueResponse {
        job_id: row.get(0),
        priority: row.get(1),
        payload: row.get(2),
    }));
}

#[get("/dequeue/<amount>")]
pub async fn dequeue_amount(
    amount: String,
    db: &rocket::State<Arc<Mutex<Client>>>,
    heap: &rocket::State<Arc<Mutex<MinHeap>>>,
    clock: &rocket::State<Arc<Mutex<u64>>>,
) -> Result<Json<BatchDequeueResponse>, ApiError> {
    let client = db.lock().await;
    let mut heap = heap.lock().await;

    if heap.heap.is_empty() {
        return Err(ApiError::EmptyHeapError);
    }

    let mut jobs: Vec<DequeueResponse> = Vec::new();

    let amount: usize = amount.parse::<usize>().map_err(|_| {
        error!("Error: Non-numerical amount provided by GET request in /dequeue/<amount>");
        ApiError::InternalServerError(format!("Provided non numerical amount"))
    })?;

    // Increment logical time
    *clock.lock().await += 1;

    for _ in 0..amount {
        let node: HeapNode = match heap.get_top() {
            Some(n) => n,
            None => return Err(ApiError::EmptyHeapError),
        };

        let query = client
            .prepare("SELECT * FROM jobs WHERE job_id = $1")
            .await
            .map_err(|_| {
                error!("Error: Failed to create SELECT query");
                ApiError::DatabaseError(format!("Error creating query"))
            })?;

        let row = client
            .query_one(&query, &[&(node.job_id as i64)])
            .await
            .map_err(|_| {
                error!("Error: Failed to run SELECT query");
                ApiError::DatabaseError(format!("Error database SELECT query failed."))
            })?;

        jobs.push(DequeueResponse {
            job_id: row.get(0),
            priority: row.get(1),
            payload: row.get(2),
        });
    }

    return Ok(Json(BatchDequeueResponse { jobs }));
}

#[post("/enqueue", format = "json", data = "<request>")]
pub async fn enqueue(
    request: Json<EnqueueRequest>,
    db: &rocket::State<Arc<Mutex<Client>>>,
    heap: &rocket::State<Arc<Mutex<MinHeap>>>,
    clock: &rocket::State<Arc<Mutex<u64>>>,
) -> Result<Json<CreationResponse>, ApiError> {
    let client = db.lock().await;

    let query = client
        .prepare("INSERT INTO jobs (priority, payload) VALUES ($1,$2) RETURNING job_id")
        .await
        .map_err(|_| {
            error!("Error: Failed to create INSERT query");
            ApiError::DatabaseError(format!("Error creating query"))
        })?;

    let row = client
        .query_one(&query, &[&request.priority, &request.payload])
        .await
        .map_err(|_| {
            error!("Error: Failed to run INSERT query");
            ApiError::DatabaseError(format!("Error creating row"))
        })?;

    let job_id: i64 = row.get(0);

    // Increment logical time
    *clock.lock().await += 1;

    heap.lock()
        .await
        .insert(request.priority as u32, job_id as u64, *clock.lock().await);

    heap.lock()
        .await
        .calculate_effective_priority(*clock.lock().await);

    println!("Inserted job with job_id {} into jobs table", job_id);

    return Ok(Json(CreationResponse {
        message: format!("Job with job_id={} successully added to database", job_id),
        job_id: job_id as u64,
    }));
}

#[post("/update", format = "json", data = "<request>")]
pub async fn update(
    request: Json<UpdateRequest>,
    db: &rocket::State<Arc<Mutex<Client>>>,
    heap: &rocket::State<Arc<Mutex<MinHeap>>>,
    clock: &rocket::State<Arc<Mutex<u64>>>,
) -> Result<Json<UpdateResponse>, ApiError> {
    let mut heap = heap.lock().await;

    heap.change_priority(request.job_id as u64, request.priority as u32);

    let client = db.lock().await;

    let _ = client
        .execute(
            "UPDATE jobs SET priority = $1 WHERE job_id = $2",
            &[&request.priority, &request.job_id],
        )
        .await
        .map_err(|_| {
            error!(
                "Error: Failed to run UPDATE query on job {}",
                request.job_id
            );
            ApiError::DatabaseError(format!("Error updating database"))
        })?;

    // Increment logical time
    *clock.lock().await += 1;

    heap.calculate_effective_priority(*clock.lock().await);

    return Ok(Json(UpdateResponse {
        message: format!(
            "Job with job_id={} has been successfully updated",
            request.job_id
        ),
    }));
}
