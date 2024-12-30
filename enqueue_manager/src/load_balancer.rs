#[allow(dead_code)]
mod load_balancer {
    use log::{error, info};
    use std::collections::VecDeque;
    use std::fmt::Display;
    use tonic::transport::Channel;

    use crate::job_management::node_health_service_client::NodeHealthServiceClient;
    use crate::job_management::{EnqueueRequest, NodeHealthRequest};

    pub struct Node {
        address: String,
        weight: f32,
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
        pub fn new(address: String, weight: f32) -> Self {
            return Node { address, weight };
        }
    }

    pub struct LoadBalancer {
        buffer: VecDeque<EnqueueRequest>,
        nodes: Vec<Node>,
        available_nodes: u32,
    }

    impl LoadBalancer {
        pub async fn new(
            available_nodes: u32,
            addresses: Vec<String>,
        ) -> Result<Self, Box<dyn std::error::Error + 'static>> {
            let mut nodes: Vec<Node> = Vec::with_capacity(available_nodes as usize);
            let mut weights: Vec<f32> = Vec::with_capacity(available_nodes as usize);
            for address in &addresses {
                let weight: f32 = Self::get_weight(&address).await?;
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

            return Ok(LoadBalancer {
                buffer: VecDeque::new(),
                nodes,
                available_nodes,
            });
        }

        /// Calculates the weight of the leader
        async fn get_weight(address: &String) -> Result<f32, Box<dyn std::error::Error + 'static>> {
            let request: NodeHealthRequest = NodeHealthRequest {};

            // log gRPC request
            info!("NodeHealthService Request to address {}", address);

            let mut client: NodeHealthServiceClient<Channel> =
                NodeHealthServiceClient::connect(address.clone()).await?;

            error!(
                "Failed to estblish connection to Node Health Service Client through address {}",
                address
            );

            let response = client.get_node_health(request.clone()).await;

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
