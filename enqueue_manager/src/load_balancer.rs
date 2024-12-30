mod load_balancer {
    use crate::buffer::Buffer;

    const WEIGHT_1: f32 = 0.3;
    const WIEGHT_2: f32 = 0.25;
    const WEIGHT_3: f32 = 0.20;
    const WEIGHT_4: f32 = 0.15;
    const WEIGHT_5: f32 = 0.10;

    pub struct LoadBalancer {
        buffers: Vec<Buffer>,
        weights: Vec<f32>,
        available_leaders: u32,
    }

    impl LoadBalancer {
        pub fn new(available_leaders: u32) -> Self {
            let weights: Vec<f32> = Vec::with_capacity(available_leaders as usize);
            let buffers: Vec<Buffer> = vec![];
            for _ in 0..5 {
                buffers.push(Buffer::new());
            }
            return LoadBalancer {
                buffers,
                weights,
                available_leaders,
            };
        }
    }
}
