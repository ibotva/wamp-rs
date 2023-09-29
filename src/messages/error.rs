use std::marker::PhantomData;

use serde_json::Value;
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde::{Serialize, Deserialize, de::Visitor};

use crate::{messages::helpers, roles::Roles};

use super::{WampMessage, MessageDirection};

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq)]
#[repr(u64)]
pub enum WampErrorEvent {
    Unsubsubscribe = 34,
    Subscribe = 32,
    Publish = 16,
    Register = 64,
    Unregister = 66
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WampError  {
    pub event: WampErrorEvent,
    pub request_id: u64,
    pub details: Value,
    pub error: String,
}

impl WampMessage for WampError {
    const ID: u64 = 8;

    fn direction(r: Roles) -> &'static MessageDirection {
        match r {
            Roles::Callee => &MessageDirection {
                receives: &true,
                sends: &true,
            },
            Roles::Caller => &MessageDirection {
                receives: &true,
                sends: &false,
            },
            Roles::Publisher => &MessageDirection {
                receives: &true,
                sends: &false,
            },
            Roles::Subscriber => &MessageDirection {
                receives: &true,
                sends: &false,
            },
            Roles::Dealer => &MessageDirection {
                receives: &true,
                sends: &true,
            },
            Roles::Broker => &MessageDirection {
                receives: &false,
                sends: &true,
            },
        }
    }
}

impl Serialize for WampError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        (Self::ID, &self.event, &self.request_id, &self.details, &self.error).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for WampError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        struct WampErrorVisitor(PhantomData<u64>, PhantomData<WampErrorEvent>, PhantomData<u64>, PhantomData<Value>, PhantomData<String>);

        impl<'vi> Visitor<'vi> for WampErrorVisitor {
            type Value = WampError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of WampError components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                 A: serde::de::SeqAccess<'vi>, 
            {   
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message id must be present and type u64.")?;
                helpers::validate_id::<WampError, A, _>(&message_id, "WampError")?;
                let event: WampErrorEvent = helpers::deser_seq_element(&mut seq, "Message type of error must be present and type u64")?;
                let request_id: u64 = helpers::deser_seq_element(&mut seq, "Request ID must be present and type u64")?;
                let details: Value = helpers::deser_seq_element(&mut seq, "Details must be present and object like")?;
                helpers::deser_value_is_object::<A, _>(&details, "Details must be object like.")?;
                let error: String = helpers::deser_seq_element(&mut seq, "Error URI must be present and type String")?;
                helpers::deser_value_is_object::<A, _>(&details, "Details must be object like.")?;
                Ok(WampError {
                    event,
                    request_id,
                    details,
                    error
                })
            }
        }

        deserializer.deserialize_struct("WampError", &["event", "request_id", "details", "error"], WampErrorVisitor(PhantomData, PhantomData, PhantomData, PhantomData, PhantomData))


    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::WampError;

    #[test]
    fn unsubscribe_test() {
        let data = r#"[8,34,85346237,{},"wamp.error.no_such_subscription"]"#;
        let un_e = WampError {
            event: super::WampErrorEvent::Unsubsubscribe,
            request_id: 85346237,
            details: serde_json::json!({}),
            error: "wamp.error.no_such_subscription".to_string()
        };
        let un_e_2: WampError = from_str(data).unwrap();
        assert_eq!(un_e, un_e_2);
        let data_2 = to_string(&un_e).unwrap();
        assert_eq!(data, data_2)
    }
}