use std::cell::RefCell;
use std::sync::Arc;

use crate::job_management::EnqueueRequest;

pub struct BufferNode {
    job: EnqueueRequest,
    next: Option<Arc<RefCell<BufferNode>>>,
}

pub struct Buffer {
    head: Option<Arc<RefCell<BufferNode>>>,
}

impl BufferNode {
    pub fn new(job: EnqueueRequest) -> Self {
        return BufferNode { job, next: None };
    }

    pub fn add_next(&mut self, next: Arc<RefCell<BufferNode>>) {
        self.next = Some(next);
    }
}

impl Clone for BufferNode {
    fn clone(&self) -> Self {
        return BufferNode {
            job: self.job.clone(),
            next: self.next.clone(),
        };
    }
}

impl Buffer {
    pub fn new() -> Self {
        return Buffer { head: None };
    }

    pub fn insert(&mut self, job: EnqueueRequest) {
        if self.head.is_none() {
            self.head = Some(Arc::new(RefCell::new(BufferNode::new(job))));
        } else {
            // unwrap is safe since its established that head is some

            let mut current: &Option<Arc<RefCell<BufferNode>>> = &(self.head);

            while current.is_some() {
                let current_unwrap = current.unwrap();
                current = &current_unwrap.borrow().next;
            }
        }
    }
}
