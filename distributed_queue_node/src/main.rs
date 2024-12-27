use std::sync::Arc;

use distributed_queue_node::api::{dequeue, dequeue_amount};
use distributed_queue_node::db::attatch_db;
use distributed_queue_node::min_heap::MinHeap;
use tokio::sync::Mutex;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    let min_heap: Arc<Mutex<MinHeap>> = Arc::new(Mutex::new(MinHeap::new()));
    rocket::build()
        .attach(attatch_db())
        .manage(min_heap)
        .mount("/", routes![dequeue, dequeue_amount])
}
