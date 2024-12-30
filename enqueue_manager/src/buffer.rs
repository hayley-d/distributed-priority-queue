use std::sync::Arc;

use crate::job_management::EnqueueRequest;

pub struct BufferNode {
    job: EnqueueRequest,
}

pub struct Buffer {
    head: Option<Arc<BufferNode>>,
}

impl Buffer {
    pub fn new() -> Self {
        return Buffer { head: None };
    }
}
