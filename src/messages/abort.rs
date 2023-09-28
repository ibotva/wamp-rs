use serde::{Serialize, de};
use serde_json::Value;

use super::{WampMessage, helpers};

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
        let details = helpers::value_is_object::<S, _>(&self.details, "Details must be object like.")?;
        (Self::ID, &self.reason, &details).serialize(serializer)
    }
}