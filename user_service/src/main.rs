use axum::Router;
use axum::routing::get;
use std::net::SocketAddr;
use std::process::exit;
use crate::message::Message;

mod message;
mod redis_subscribe;
mod redis_publish;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(error) = redis_subscribe::subscribe(String::from("who_is_there")) {
        println!("{:?}", error);
        panic!("{:?}", error);
    } else {
        println!("connected to queue");
    }

    let app = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/kill", get(kill));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn root() -> &'static str { "Jeg æder blåbærsyltetøj!" }

async fn hello() -> &'static str {
    let channel = "who_is_there".to_string();
    let payload = "Who is there?".to_string();
    let message = Message::new(channel, payload);
    println!("id: {}, channel: {}, payload: {}", message.id, message.channel, message.payload);

    redis_publish::publish_message(message).unwrap();
    "Jeg siger goddag!"
}

async fn kill() { exit(1) }
