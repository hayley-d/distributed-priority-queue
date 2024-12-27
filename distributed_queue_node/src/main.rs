use distributed_queue_node::api::{dequeue, dequeue_amount};
use distributed_queue_node::db::attatch_db;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(attatch_db())
        .mount("/", routes![dequeue, dequeue_amount])
}
