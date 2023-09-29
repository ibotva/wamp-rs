use std::fmt::Formatter;
use std::marker::PhantomData;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{SeqAccess, Visitor};
use serde_json::{json, Value};

use crate::roles::Roles;

use super::{helpers, WampMessage, MessageDirection};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WampResult {
    pub request_id: u64,
    pub details: Value,
    pub args: Value,
    pub kwargs: Value
}

impl WampMessage for WampResult {
    const ID: u64 = 50;

    fn direction(role: Roles) -> &'static MessageDirection {
        match role {
            Roles::Callee => &MessageDirection {
                receives: &false,
                sends: &false,
            },
            Roles::Caller => &MessageDirection {
                receives: &true,
                sends: &false,
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
                sends: &false,
            },
            Roles::Broker => &MessageDirection {
                receives: &false,
                sends: &true,
            },
        }
    }
}

impl Serialize for WampResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let details = helpers::ser_value_is_object::<S, _>(&self.details, "details must be object like.")?;
        let args = helpers::ser_value_is_args::<S, _>(&self.args, "Args must be Array like or Null.")?;
        let kwargs = helpers::ser_value_is_kwargs::<S, _>(&self.kwargs, "Kwargs must be Object like or Null.")?;
        if args.is_null() {
            if kwargs.is_null() {
                (Self::ID, &self.request_id, details).serialize(serializer)
            } else {
                (Self::ID, &self.request_id, details, json!([]), kwargs).serialize(serializer)
            }
        } else {
            if kwargs.is_null() {
                (Self::ID, &self.request_id, details, args).serialize(serializer)
            } else {
                (Self::ID, &self.request_id, details, args, kwargs).serialize(serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for WampResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct WampResultVisitor(PhantomData<u64>, PhantomData<u64>, PhantomData<Value>, PhantomData<Value>, PhantomData<Value>);
        
        impl<'vi> Visitor<'vi> for WampResultVisitor {
            type Value = WampResult;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of WampResult components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'vi> {
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message ID must be present and type u8.")?;
                helpers::validate_id::<WampResult, A, _>(&message_id, "WampResult")?;
                let request_id: u64 = helpers::deser_seq_element(&mut seq, "Request ID must be present and type u64.")?;
                let details: Value = helpers::deser_seq_element(&mut seq, "details must be present and object like.")?;
                helpers::deser_value_is_object::<A, _>(&details, "details must be object like.")?;
                let args: Value = helpers::deser_seq_element(&mut seq, "Args must be array like or null.")?;
                let kwargs: Value = helpers::deser_seq_element(&mut seq, "Kwargs must be object like or null.")?;
                Ok(WampResult {
                    request_id,
                    details,
                    args,
                    kwargs
                })
            }
        }

        deserializer.deserialize_struct("WampResult", &["request_id", "details", "args", "kwargs"], WampResultVisitor(PhantomData, PhantomData, PhantomData, PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, from_str, to_string};

    use super::WampResult;

    #[test]
    fn test() {
        let d1 = r#"[50,7814135,{},[],{"karma":10,"userid":123}]"#;
        let mut w1 = WampResult {
            request_id: 7814135,
            details: json!({}),
            args: serde_json::Value::Null,
            kwargs: json!({"userid":123,"karma":10})
        };
        assert_ne!(from_str::<WampResult>(d1).unwrap(), w1);
        w1.args = json!([]);
        assert_eq!(from_str::<WampResult>(d1).unwrap(), w1);
        assert_eq!(to_string(&w1).unwrap(), d1);
    }
}