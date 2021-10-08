extern crate sse_client;
use sse_client::EventSource;

fn main() {
    let event_source = EventSource::new("http://127.0.0.1:3030/ticks").unwrap();

    for event in event_source.receiver().iter() {
        println!("New Message: {}", event.data);
    }
}
