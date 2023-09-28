use serde::Serialize;
use serde_json::Value;

use super::WampMessage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abort {
    pub reason: String,
    pub details: Value
}

impl WampMessage for Abort {
    const ID: u8 = 3;
}

impl Serialize for Abort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        todo!()
    }
}