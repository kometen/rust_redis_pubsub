mod message;
mod redis_publish;
mod redis_subscribe;

use std::net::SocketAddr;
use std::process::exit;
use axum::{
    response::IntoResponse,
    routing::{get},
    Router
};
use crate::message::Message;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/kill", get(kill));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn root() -> &'static str { "Jeg æder blåbærsyltetøj!" }

async fn hello() -> impl IntoResponse {
    let channel = "who_is_there".to_string();
    let payload = "Who is there?".to_string();
    let message = Message::new(channel, payload);
    println!("id: {}, channel: {}, payload: {}", message.id, message.channel, message.payload);

    redis_publish::publish_message(message).unwrap();
    let response: String = redis_subscribe::subscribe("i_am_here".to_string());
    response
}

async fn kill() { exit(1) }
