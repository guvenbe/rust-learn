use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub text: String,
}

pub fn greet(name: &str) -> Message {
    Message { text: format!("Hello, {}!", name) }
}