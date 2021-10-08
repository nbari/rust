extern crate sse_client;
use sse_client::EventSource;

fn main() {
    let event_source = EventSource::new("http://46.4.105.131:9000/ticks").unwrap();

    for event in event_source.receiver().iter() {
        println!("New Message: {}", event.data);
    }
}
