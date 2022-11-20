extern crate redis;

pub fn subscribe(channel: String) -> String {
    let client = redis::Client::open("redis://redis").unwrap();
    let mut con = client.get_connection().unwrap();
    let mut pubsub = con.as_pubsub();
    pubsub.subscribe(channel).unwrap();
    let msg = pubsub.get_message().unwrap();
    let payload : String = msg.get_payload().unwrap();

    payload
}