use std::fmt::Formatter;
use std::marker::PhantomData;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{SeqAccess, Visitor};
use serde_json::{json, Value};

use crate::roles::Roles;

use super::{helpers, WampMessage, MessageDirection};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invocation {
    pub request_id: u64,
    pub registration: u64,
    pub details: Value,
    pub args: Value,
    pub kwargs: Value
}

impl WampMessage for Invocation {
    const ID: u64 = 68;

    fn direction(role: Roles) -> &'static MessageDirection {
        match role {
            Roles::Callee => &MessageDirection {
                receives: &true,
                sends: &false,
            },
            Roles::Caller => &MessageDirection {
                receives: &false,
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
                sends: &true,
            },
            Roles::Broker => &MessageDirection {
                receives: &false,
                sends: &false,
            },
        }
    }
}

impl Serialize for Invocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        
        let args = helpers::ser_value_is_args::<S, _>(&self.args, "Args must be Array like or Null.")?;
        let kwargs = helpers::ser_value_is_kwargs::<S, _>(&self.kwargs, "Kwargs must be Object like or Null.")?;


        if args.is_null() {
            if kwargs.is_null() {
                (Self::ID, &self.request_id, &self.registration, &self.details).serialize(serializer)
            } else {
                (Self::ID, &self.request_id, &self.registration, &self.details, json!([]), kwargs).serialize(serializer)
            }
        } else {
            if kwargs.is_null() {
                (Self::ID, &self.request_id, &self.registration, &self.details, args).serialize(serializer)
            } else {
                (Self::ID, &self.request_id, &self.registration, &self.details, args, kwargs).serialize(serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for Invocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct InvocationVisitor(PhantomData<u8>, PhantomData<u64>, PhantomData<u64>, PhantomData<Value>, PhantomData<Value>, PhantomData<Value>);
        
        impl<'vi> Visitor<'vi> for InvocationVisitor {
            type Value = Invocation;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Invocation components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'vi> {
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message ID must be present and type u8.")?;
                helpers::validate_id::<Invocation, A, _>(&message_id, "Invocation")?;
                let request_id: u64 = helpers::deser_seq_element(&mut seq, "request_id must be present and type u64.")?;
                let registration: u64 = helpers::deser_seq_element(&mut seq, "registration must be present and object like.")?;
                let details: Value = helpers::deser_seq_element(&mut seq, "Details must be present and object like.")?;
                helpers::deser_value_is_object::<A, _>(&details, "Details must be object like.")?;
                let args: Value = helpers::deser_seq_element(&mut seq, "Args must be array like or null.")?;
                let kwargs: Value = helpers::deser_seq_element(&mut seq, "Kwargs must be object like or null.")?;
                Ok(Invocation {
                    request_id,
                    registration,
                    details,
                    args,
                    kwargs
                })
            }
        }

        deserializer.deserialize_struct("Invocation", &["request_id", "registration", "details", "args", "kwargs"], InvocationVisitor(PhantomData, PhantomData, PhantomData, PhantomData, PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::Invocation;

    #[test]
    fn test() {
        let d = r#"[68,6131533,9823529,{},["johnny"],{"firstname":"John","surname":"Doe"}]"#;
        let mut ed = Invocation {
            request_id: 6131533,
            registration: 9823529,
            details: serde_json::json!({}),
            args: serde_json::Value::Null,
            kwargs: serde_json::json!({"firstname":"John","surname":"Doe"})
        };
        let ed2: Invocation = from_str(d).unwrap();
        assert_ne!(ed, ed2);
        ed.args = serde_json::json!(["johnny"]);
        assert_eq!(ed, ed2);
        let d2 = to_string(&ed).unwrap();
        assert_eq!(d, d2);
    }
}