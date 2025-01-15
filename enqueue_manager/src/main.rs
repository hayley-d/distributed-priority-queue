use enqueue_manager::job_management::EnqueueRequest;
use enqueue_manager::load_balancer::load_balancer::LoadBalancer;
use enqueue_manager::manager_state::ManagerState;
use log::error;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> Rocket<Build> {
    let mut nodes: Vec<String> = vec!["http://node1".to_string(), "http://node2".to_string()];
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let state = ManagerState::new(nodes.clone());
    let load_balancer: LoadBalancer = match LoadBalancer::new(&mut nodes).await {
        Ok(lb) => lb,
        Err(_) => {
            error!(target:"error_logger", "Failed to start server, issue creating load balancer");
            std::process::exit(1);
        }
    };

    rocket::build()
        .manage(state)
        .manage(load_balancer)
        .mount("/", routes![enqueue])
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnqueueJobRequest {
    priority: i32,
    payload: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnqueueResponse {
    message: String,
}

#[post("/enqueue", format = "json", data = "<request>")]
pub async fn enqueue(
    request: Json<EnqueueJobRequest>,
    manager_state: &rocket::State<Arc<Mutex<ManagerState>>>,
    load_balancer: &rocket::State<Arc<Mutex<LoadBalancer>>>,
) -> Result<Json<EnqueueResponse>, Status> {
    let enqueue_request: EnqueueRequest = EnqueueRequest {
        priority: request.priority,
        payload: request.payload.clone(),
    };

    let mut state = manager_state.lock().await;
    let mut load_bal = load_balancer.lock().await;

    state.increment_time();
    load_bal.insert(enqueue_request);

    Ok(Json(EnqueueResponse {
        message: "Job successfully added to queue".to_string(),
    }))
}
