pub struct BufferNode {
    job: EnqueueRequest,
}

pub struct Buffer {
    head: Arc<BufferNode>,
}
