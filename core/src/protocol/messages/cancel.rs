use std::marker::PhantomData;
use serde::{Serialize, de::{self, Visitor}, Deserialize};
use serde_json::Value;
use crate::protocol::roles::Roles;
use super::{WampMessage, helpers, MessageDirection};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cancel {
    pub request_id: u64,
    pub options: Value
}

#[macro_export]
macro_rules! cancel {
    () => {
        cancel!(serde_json::json!({}))
    };
    ($options:expr) => {
        Cancel {
            request_id: $crate::protocol::increment(),
            options: $options
        }
    };
}

impl WampMessage<Cancel> for Cancel {
    const ID: u64 = 49;

    fn direction(role: Roles) -> &'static MessageDirection {
        match role {
            Roles::Callee => &MessageDirection {
                receives: &false,
                sends: &false,
            },
            Roles::Caller => &MessageDirection {
                receives: &false,
                sends: &true,
            },
            Roles::Publisher => &MessageDirection {
                receives: &false,
                sends: &false,
            },
            Roles::Subscriber => &MessageDirection {
                receives: &false,
                sends: &false,
            },
            Roles::Dealer => &MessageDirection {
                receives: &false,
                sends: &true,
            },
            Roles::Broker => &MessageDirection {
                receives: &false,
                sends: &false,
            },
        }
    }
}

impl Serialize for Cancel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let options = helpers::ser_value_is_object::<S, _>(&self.options, "Options must be object like.")?;
        (Self::ID, &self.request_id, options).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Cancel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where 
        D: serde::Deserializer<'de> 
    {
        struct CancelVisitor(PhantomData<u8>, PhantomData<String>, PhantomData<Value>);

        impl<'vi> Visitor<'vi> for CancelVisitor {
            type Value = Cancel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("WAMP Cancel frame, expressed as a sequence.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'vi>, 
            {
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message ID must be type u64.")?;
                helpers::validate_id::<Cancel, A, _>(&message_id, "Cancel")?;
                let request_id: u64 = helpers::deser_seq_element(&mut seq, "Request ID must be a u64.")?;
                let options: Value = helpers::deser_seq_element(&mut seq, "Options must be a JSON value.")?;
                helpers::deser_value_is_object::<A, _>(&options, "Options must be object like.")?;
                Ok(Cancel { request_id, options })
            }
        }
        
        deserializer.deserialize_struct("Cancel", &["request_id", "options"], CancelVisitor(PhantomData, PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, json, to_string};

    use super::*;

    #[test]
    fn test() {
        let d1 = r#"[49,9129132,{}]"#;
        let c1 = Cancel {
            request_id: 9129132,
            options: json!({})
        };
        assert_eq!(c1, from_str(d1).unwrap());
        assert_eq!(d1, to_string(&c1).unwrap());
    }
}