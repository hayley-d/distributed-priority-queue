#[allow(dead_code)]
pub mod load_balancer {
    use log::{error, info};
    use std::collections::VecDeque;
    use std::fmt::Display;
    use tokio::time::{timeout, Duration};
    use tonic::transport::Channel;

    use crate::job_management::job_service_client::JobServiceClient;
    use crate::job_management::node_health_service_client::NodeHealthServiceClient;
    use crate::job_management::{EnqueueRequest, Job, NodeHealthRequest};

    /// Node represents a replica in the distributed system
    /// `address` is url address of the replica to recieved gRPC requests
    /// `weight` is the calculated weight for the node based on CPU utilization, memory usage,queue
    /// depth and response time
    #[derive(Debug)]
    pub struct Node {
        address: String,
        weight: f32,
    }

    impl Clone for Node {
        fn clone(&self) -> Self {
            Node {
                address: self.address.clone(),
                weight: self.weight.clone(),
            }
        }
    }

    impl Eq for Node {}

    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.weight == other.weight && self.address == other.address
        }
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(&other))
        }
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            if self.weight < other.weight {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        }
    }

    impl Node {
        /// Creates a new node
        ///
        /// # Arguments
        /// `address`: The url address of the node.
        /// `weight`: The calculated weight of the node.
        ///
        /// # Returns
        /// A new node.
        pub fn new(address: String, weight: f32) -> Self {
            Node { address, weight }
        }
    }

    /// Load Blanacer State structure that keeps information needed to distribute jobs.
    /// `buffer`: The buffer jobs are added to before being distributed.
    /// `nodes`: A vector of nodes in the distributed system.
    /// `lamport_timestamp`: The logical clock.
    pub struct LoadBalancer {
        buffer: VecDeque<EnqueueRequest>,
        nodes: Vec<Node>,
        lamport_timestamp: u64,
    }

    impl LoadBalancer {
        /// Increments the logical clock of the load balancer and returns the current clock count.
        pub fn increment_time(&mut self) -> u64 {
            let temp = self.lamport_timestamp;
            self.lamport_timestamp += 1;
            temp
        }

        /// Inserts a job into the buffer of the load balancer
        pub fn insert(&mut self, job: EnqueueRequest) {
            self.buffer.push_back(job);
        }

        /// Creates a new load balancer state.
        ///
        /// # Arguments
        /// `addresses`: A vector of url addresses to the nodes in the distributed system.
        ///
        /// # Returns
        /// A Result object which is either an Ok(LoadBalancer) or an Err(Box<dyn
        /// std::error::Error>)
        pub async fn new(
            addresses: &mut Vec<String>,
        ) -> Result<Self, Box<dyn std::error::Error + 'static>> {
            let mut nodes: Vec<Node> = Vec::with_capacity(addresses.len());
            let mut weights: Vec<f32> = Vec::with_capacity(addresses.len());

            for (i, address) in addresses.clone().iter().enumerate() {
                let weight: f32 = match Self::get_weight(&address).await {
                    Ok(w) => w,
                    Err(_) => {
                        error!(target: "error_logger","Failed to get response for node health from address: {}",address);
                        addresses.remove(i);
                        continue;
                    }
                };
                weights.push(weight);
            }

            let total_weight: f32 = weights.iter().sum();

            let normalized_weights: Vec<f32> = weights
                .iter()
                .map(|&w| (w / total_weight) * 100.0)
                .collect();

            for (i, weight) in normalized_weights.iter().enumerate() {
                nodes.push(Node::new(addresses[i].clone(), *weight));
            }

            nodes.sort_by(|a, b| b.cmp(&a));

            Ok(LoadBalancer {
                buffer: VecDeque::new(),
                nodes,
                lamport_timestamp: 0,
            })
        }

        /// Adds a node to the load balancer
        ///
        /// # Arguments
        /// `address`: The url address of the node.
        async fn add_node(&mut self, address: String) {
            let weight = match Self::get_weight(&address).await {
                Ok(w) => w,
                Err(_) => {
                    error!(target: "error_logger","Failed to get node health status from address: {}", address);
                    return;
                }
            };

            let mut weights: Vec<f32> = Vec::with_capacity(self.nodes.len() + 1);

            let mut remove_nodes: Vec<usize> = Vec::new();
            for (i, node) in self.nodes.iter().enumerate() {
                let weight = match Self::get_weight(&node.address).await {
                    Ok(w) => w,
                    Err(_) => {
                        error!(
                            "Failed to get node health status from address: {}",
                            node.address
                        );
                        self.available_nodes -= 1;
                        remove_nodes.push(i);
                        continue;
                    }
                };

                weights.push(weight);
            }

            for i in remove_nodes {
                self.nodes.remove(i);
            }

            let total_weight = weights.iter().sum::<f32>();
            weights.push(weight);
            self.nodes.push(Node::new(address, weight));

            let normalized_weights: Vec<f32> = weights
                .iter()
                .map(|&w| (w / total_weight) * 100.0)
                .collect();

            for (i, weight) in normalized_weights.iter().enumerate() {
                self.nodes[i].weight = *weight;
            }
        }

        async fn update_weighting(&mut self) {
            let mut weights: Vec<f32> = Vec::with_capacity(self.available_nodes as usize + 1);

            let mut remove_nodes: Vec<usize> = Vec::new();
            for (i, node) in self.nodes.iter().enumerate() {
                let weight = match Self::get_weight(&node.address).await {
                    Ok(w) => w,
                    Err(_) => {
                        error!(
                            "Failed to get node health status from address: {}",
                            node.address
                        );
                        self.available_nodes -= 1;
                        remove_nodes.push(i);
                        continue;
                    }
                };

                weights.push(weight);
            }

            for i in remove_nodes {
                self.nodes.remove(i);
            }

            let total_weight = weights.iter().sum::<f32>();

            let normalized_weights: Vec<f32> = weights
                .iter()
                .map(|&w| (w / total_weight) * 100.0)
                .collect();

            for (i, weight) in normalized_weights.iter().enumerate() {
                if self.nodes.get(i).is_some() {
                    self.nodes[i].weight = *weight;
                }
            }
        }

        /// Calculates the weight of the leader
        async fn get_weight(address: &String) -> Result<f32, Box<dyn std::error::Error + 'static>> {
            let request: NodeHealthRequest = NodeHealthRequest {};

            // log gRPC request
            info!("NodeHealthService Request to address {}", address);

            let mut client: NodeHealthServiceClient<Channel> =
                NodeHealthServiceClient::connect(address.clone()).await?;

            let response = match timeout(
                Duration::from_millis(10),
                client.get_node_health(request.clone()),
            )
            .await
            {
                Ok(value) => value,
                Err(_) => {
                    error!(
                        "Failed to get response from node at {} request timeout",
                        address
                    );
                    return Err(Box::new(RpcError::FailedRequest));
                }
            };

            match response {
                Ok(res) => {
                    let res = res.into_inner().clone();
                    let (cpu_utilization, memory_usage, queue_depth, _) = (
                        &res.cpu_utilization,
                        &res.memory_usage,
                        &res.queue_depth,
                        &res.response_time,
                    );

                    let weight: f32 = (((1.0 - cpu_utilization) / 100.0)
                        * ((1.0 - memory_usage) / 100.0)
                        * ((1.0 - (*queue_depth as f32)) / 100.0))
                        .round()
                        / 100.0;

                    return Ok(weight);
                }
                Err(_) => {
                    error!(
                        "Failed to obtain node health status from node at {}",
                        address
                    );
                    return Err(Box::new(RpcError::FailedRequest));
                }
            }
        }

        pub async fn distribute(&mut self) -> Result<(), Box<dyn std::error::Error + 'static>> {
            let number_jobs: usize = self.nodes.len();

            for node in self.nodes.clone() {
                let my_jobs: i32 = (number_jobs as f32 * node.weight).floor() as i32;

                for _ in 0..my_jobs {
                    let time = self.increment_time();
                    if time % 100 == 0 {
                        self.update_weighting().await;
                    }
                    let enqueue_request: EnqueueRequest = match self.buffer.pop_front() {
                        Some(r) => r,
                        None => {
                            error!("Buffer is empty");
                            return Err(Box::new(RpcError::FailedRequest));
                        }
                    };

                    info!("Job Service Request to address {}", node.address);

                    let mut client: JobServiceClient<Channel> =
                        JobServiceClient::connect(node.address.clone()).await?;

                    let response = match timeout(
                        Duration::from_millis(10),
                        client.enqueue_job(enqueue_request),
                    )
                    .await
                    {
                        Ok(value) => value,
                        Err(_) => {
                            error!(
                                "Failed to get response from node at {} request timeout",
                                node.address
                            );
                            return Err(Box::new(RpcError::FailedRequest));
                        }
                    };

                    match response {
                        Ok(_) => (),
                        Err(_) => {
                            error!(
                                "Failed to obtain enqueue response from node at {}",
                                node.address
                            );
                            return Err(Box::new(RpcError::FailedRequest));
                        }
                    }
                }
            }
            return Ok(());
        }
    }

    #[derive(Debug)]
    pub enum RpcError {
        FailedRequest,
    }

    impl std::error::Error for RpcError {}

    impl Display for RpcError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "gRPC error")
        }
    }
}

#[cfg(test)]
mod tests {}
