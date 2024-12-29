use std::sync::Arc;

use leader::grpc::LocalJobService;
use leader::job_management::job_service_server::JobServiceServer;
use leader::node_state::NodeState;
use tokio::sync::Mutex;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize NodeState asynchronously
    let followers = vec!["http://follower1", "http://follower2"]; // Example list of followers

    let node_state = NodeState::new(followers);

    // Initialize the job service with the node_state
    let job_service = LocalJobService::new(node_state).await;

    let addr = "[::1]:50051".parse()?;
    let svc = JobServiceServer::new(job_service);

    println!("Leader service listening on {:?}", addr);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
