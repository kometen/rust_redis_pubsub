use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub channel: String,
    pub payload: String
}

impl Message {
    pub fn new(channel: String, payload: String) -> Message {
        Message {
            id: Message::generate_id(),
            channel,
            payload
        }
    }

    fn generate_id() -> String {
        Uuid::new_v4().to_string()
    }
}
