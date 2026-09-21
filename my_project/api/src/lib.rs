use core::greet;
use serde_json;

pub fn api_greet() -> String {
    let msg = greet("from API");
    serde_json::to_string(&msg).unwrap()
}