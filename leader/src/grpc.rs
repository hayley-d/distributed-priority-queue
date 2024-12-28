use proto::job_management::{
    EnqueueRequest, Job, JobRequest, JobResponse, JobService, JobServiceServer,
};
use tonic::{transport::Server, Request, Response, Status};


struct EnqueueRequest {
    priority: u32,
    payload: Vec<u8>,
}

struct JobResponse {
    job_id: u64,
}

#[derive(Default)]
pub struct MyJobService;

#[tonic::async_trait]
impl JobService for MyJobService {
    // EnqueueJob RPC method
    async fn enqueue_job(
        &self,
        request: Request<EnqueueRequest>,
    ) -> Result<Response<JobResponse>, Status> {
        let enqueue_request = request.into_inner();
        let priority = enqueue_request.priority;
        let payload = enqueue_request.payload;

        let paxos_prepare = PaxosPrepare {
            proposal_number: 
        };

        Ok(Response::new(JobResponse { job_id })) // Send back the job
    }
}
