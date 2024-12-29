use queue_consumer::consumer_state::ConsumerState;

fn main() {
    let nodes = vec!["http://node1", "http://node2"]; // Example list of followers

    let consumer_state = ConsumerState::new(nodes);

    println!("Hello, world!");
}
