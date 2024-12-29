use crate::job_management::{
    PaxosAccept, PaxosCommit, PaxosPrepare, PaxosPromise, PaxosPropose, PaxosService,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct PaxosState {
    pub promised_proposal: i32,
    pub accepted_proposal: i32,
    pub accepted_value: Option<Job>, // Holds the accepted job (if any)
}

impl PaxosState {
    pub fn new() -> Self {
        PaxosState {
            promised_proposal: 0,
            accepted_proposal: 0,
            accepted_value: None,
        }
    }
}

#[derive(Debug)]
pub struct PaxosServiceImpl {
    pub state: Arc<Mutex<PaxosState>>, // Shared state for Paxos
}

#[tonic::async_trait]
impl PaxosService for PaxosServiceImpl {
    async fn prepare(
        &self,
        request: Request<PaxosPrepare>,
    ) -> Result<Response<PaxosPromise>, Status> {
        let mut state = self.state.lock().await;
        let prepare = request.into_inner();

        // Check if the proposal number is greater than or equal to the promised proposal number
        if prepare.proposal_number >= state.promised_proposal {
            state.promised_proposal = prepare.proposal_number;
            Ok(Response::new(PaxosPromise {
                accepted_value: state
                    .accepted_value
                    .as_ref()
                    .map(|job| job.priority)
                    .unwrap_or(0),
            }))
        } else {
            Err(Status::failed_precondition(
                "Proposal number is less than promised.",
            ))
        }
    }

    async fn propose(
        &self,
        request: Request<PaxosPropose>,
    ) -> Result<Response<PaxosAccept>, Status> {
        let mut state = self.state.lock().await;
        let propose = request.into_inner();

        // Accept the proposal if the proposal number matches the promised proposal number
        if propose.proposal_number >= state.promised_proposal {
            state.accepted_proposal = propose.proposal_number;
            state.accepted_value = Some(propose.proposed_job.clone());
            Ok(Response::new(PaxosAccept { accepted: true }))
        } else {
            Err(Status::failed_precondition(
                "Proposal number is less than promised.",
            ))
        }
    }

    async fn commit(&self, request: Request<PaxosCommit>) -> Result<Response<()>, Status> {
        let mut state = self.state.lock().await;
        let commit = request.into_inner();

        // Commit the accepted value if the commit is true
        if commit.commit {
            if let Some(job) = &state.accepted_value {
                // Simulate storing the job in a database
                println!("Committing job: {:?}", job);
                // You could add actual database logic here
                state.accepted_value = None; // Clear the accepted value after commit
                Ok(Response::new(()))
            } else {
                Err(Status::failed_precondition("No accepted value to commit."))
            }
        } else {
            Err(Status::failed_precondition("Commit flag is false."))
        }
    }
}
