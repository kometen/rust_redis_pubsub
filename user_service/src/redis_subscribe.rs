extern crate redis;

use std::env;
use std::error::Error;
use redis::{ControlFlow, PubSubCommands};
use dotenv::dotenv;
use crate::message::Message;
use crate::redis_publish;

pub fn subscribe(channel: String) -> Result<(), Box<dyn Error>> {
    let _ = tokio::spawn(async move {
        let name = env::var("NAME").expect("Please configure name");
        let client = redis::Client::open("redis://redis").unwrap();

        let mut con = client.get_connection().unwrap();

        let _: () = con.subscribe(&[channel], |msg| {
            let received: String = msg.get_payload().unwrap();
            let message_obj = serde_json::from_str::<Message>(&received).unwrap();

            println!("{:?}", message_obj);

            let channel = "i_am_here".to_string();
            let payload = format!("I am {}.", name);
            let message = Message::new(channel, payload);
            println!("id: {}, channel: {}, payload: {}", message.id, message.channel, message.payload);

            redis_publish::publish_message(message).unwrap();

            return ControlFlow::Continue;
        }).unwrap();
    });

    Ok(())
}