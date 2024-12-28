use std::sync::Arc;

use follower::api::{dequeue, dequeue_amount};
use follower::db::attatch_db;
use follower::min_heap::MinHeap;
use follower::request_logger::RequestLogger;
use tokio::sync::Mutex;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    // initialize logging using the config file
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let min_heap: Arc<Mutex<MinHeap>> = Arc::new(Mutex::new(MinHeap::new(0.5)));
    let lamport_clock: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
    let node_id: u64 = match std::env::args().collect::<Vec<String>>().get(1) {
        Some(id) => match id.parse::<u64>() {
            Ok(i) => i,
            Err(_) => std::process::exit(1),
        },
        None => std::process::exit(1),
    };

    rocket::build()
        .attach(attatch_db())
        .attach(RequestLogger)
        .manage(min_heap)
        .manage(lamport_clock)
        .manage(Arc::new(node_id))
        .mount("/", routes![dequeue, dequeue_amount])
}
