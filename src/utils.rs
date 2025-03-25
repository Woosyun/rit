use serde::{Serialize, Deserialize};
use serde_json;

pub fn decode<'a, O: Serialize>(o: &O) -> serde_json::Result<String> {
    serde_json::to_string(o)
}

pub fn encode<'a, O: Deserialize<'a>>(content: &'a str) -> serde_json::Result<O> {
    serde_json::from_str(content)
}
