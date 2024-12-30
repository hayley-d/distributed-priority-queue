use std::sync::Arc;

use enqueue_manager::load_balancer::load_balancer::LoadBalancer;
use enqueue_manager::manager_state::ManagerState;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> Rocket<Build> {
    let mut nodes: Vec<String> = vec!["http://node1".to_string(), "http://node2".to_string()];
    let size: u32 = 2;

    let state = ManagerState::new(nodes.clone());
    let load_balancer: LoadBalancer = match LoadBalancer::new(size, &mut nodes).await {
        Ok(lb) => lb,
        Err(_) => {
            error!("Failed to start server, issue creating load balancer");
            std::process::exit(1);
        }
    };
    rocket::build()
        .manage(state)
        .manage(load_balancer)
        .mount("/", routes![enqueue])
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnqueueJobRequest {}
#[derive(Debug, Serialize, Deserialize)]
pub struct EnqueueResponse {}

#[post("/enqueue", format = "json", data = "<request>")]
pub async fn enqueue(
    request: Json<EnqueueJobRequest>,
    manager_state: &rocket::State<Arc<Mutex<ManagerState>>>,
    load_balancer: &rocket::State<Arc<Mutex<LoadBalancer>>>,
) -> Result<Json<EnqueueResponse>, Status> {
    todo!()
}
