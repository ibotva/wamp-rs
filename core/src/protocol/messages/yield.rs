use std::fmt::Formatter;
use std::marker::PhantomData;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{SeqAccess, Visitor};
use serde_json::{json, Value};
use crate::protocol::roles::Roles;
use super::{helpers, WampMessage, MessageDirection};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Yield {
    pub request_id: u64,
    pub options: Value,
    pub args: Value,
    pub kwargs: Value
}

#[macro_export]
macro_rules! r#yield {
    ($request_id:expr) => {
        r#yield!{$request_id:expr, json!({}), Value::Null, Value::Null}
    };
    ($request_id:expr, $details:expr) => {
        r#yield!{$request_id:expr, $details, Value::Null, Value::Null}
    };
    ($request_id:expr, $details:expr, args: $args:expr) => {
        r#yield!{$request_id:expr, $details, $args, Value::Null}
    };
    ($request_id:expr, $details:expr, kwargs: $kwargs:expr) => {
        r#yield!$request_id:expr, {$details, Value::Null, $kwargs}
    };
    ($request_id:expr, $details:expr, $args:expr, $kwargs:expr) => {
        Yield {
            args: $args,
            request_id: $request_id:expr,
            details: $details,
            kwargs: $kwargs
        }
    };
}


impl WampMessage<Yield> for Yield {
    const ID: u64 = 70;

    fn direction(role: Roles) -> &'static MessageDirection {
        match role {
            Roles::Callee => &MessageDirection {
                receives: &false,
                sends: &true,
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
                receives: &true,
                sends: &false,
            },
            Roles::Broker => &MessageDirection {
                receives: &false,
                sends: &false,
            },
        }
    }
}

impl Serialize for Yield {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let options = helpers::ser_value_is_object::<S, _>(&self.options, "options must be object like.")?;
        let args = helpers::ser_value_is_args::<S, _>(&self.args, "Args must be Array like or Null.")?;
        let kwargs = helpers::ser_value_is_kwargs::<S, _>(&self.kwargs, "Kwargs must be Object like or Null.")?;
        if args.is_null() {
            if kwargs.is_null() {
                (Self::ID, &self.request_id, options).serialize(serializer)
            } else {
                (Self::ID, &self.request_id, options, json!([]), kwargs).serialize(serializer)
            }
        } else {
            if kwargs.is_null() {
                (Self::ID, &self.request_id, options, args).serialize(serializer)
            } else {
                (Self::ID, &self.request_id, options, args, kwargs).serialize(serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for Yield {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct YieldVisitor(PhantomData<u64>, PhantomData<u64>, PhantomData<Value>, PhantomData<Value>, PhantomData<Value>);
        
        impl<'vi> Visitor<'vi> for YieldVisitor {
            type Value = Yield;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Yield components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'vi> {
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message ID must be present and type u8.")?;
                helpers::validate_id::<Yield, A, _>(&message_id, "Yield")?;
                let request_id: u64 = helpers::deser_seq_element(&mut seq, "Request ID must be present and type u64.")?;
                let options: Value = helpers::deser_seq_element(&mut seq, "options must be present and object like.")?;
                helpers::deser_value_is_object::<A, _>(&options, "options must be object like.")?;
                let args: Value = helpers::deser_args_kwargs_element(&mut seq, "Args must be array like or null.")?;
                let kwargs: Value = helpers::deser_args_kwargs_element(&mut seq, "Kwargs must be object like or null.")?;
                Ok(Yield {
                    request_id,
                    options,
                    args,
                    kwargs
                })
            }
        }

        deserializer.deserialize_struct("Yield", &["request_id", "options", "args", "kwargs"], YieldVisitor(PhantomData, PhantomData, PhantomData, PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, from_str, to_string};

    use super::Yield;

    #[test]
    fn test() {
        let d1 = r#"[70,6131533,{},[],{"karma":10,"userid":123}]"#;
        let mut w1 = Yield {
            request_id: 6131533,
            options: json!({}),
            args: serde_json::Value::Null,
            kwargs: json!({"userid":123,"karma":10})
        };
        assert_ne!(from_str::<Yield>(d1).unwrap(), w1);
        w1.args = json!([]);
        assert_eq!(from_str::<Yield>(d1).unwrap(), w1);
        assert_eq!(to_string(&w1).unwrap(), d1);
    }
}