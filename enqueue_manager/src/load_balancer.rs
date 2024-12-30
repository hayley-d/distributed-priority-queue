mod load_balancer {
    use std::collections::VecDeque;

    use crate::job_management::EnqueueRequest;

    pub struct LoadBalancer {
        buffer: VecDeque<EnqueueRequest>,
        weights: Vec<(String, f32)>,
        available_leaders: u32,
    }

    impl LoadBalancer {
        pub fn new(available_leaders: u32, leaders: Vec<String>) -> Self {
            let weights: Vec<(String, f32)> = Vec::with_capacity(available_leaders as usize);

            return LoadBalancer {
                buffer: VecDeque::new(),
                weights,
                available_leaders,
            };
        }
    }
}
