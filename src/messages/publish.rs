use std::fmt::Formatter;
use std::marker::PhantomData;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{SeqAccess, Visitor};
use serde_json::{json, Value};

use crate::roles::Roles;

use super::{helpers, WampMessage, MessageDirection};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Publish {
    pub request_id: u64,
    pub options: Value,
    pub topic: String,
    pub args: Value,
    pub kwargs: Value
}

impl WampMessage for Publish {
    const ID: u64 = 16;

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
                sends: &true,
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
                receives: &true,
                sends: &false,
            }
        }
    }
}

impl Serialize for Publish {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let options = helpers::ser_value_is_object::<S, _>(&self.options, "Options must be object like.")?;
        let args = helpers::ser_value_is_args::<S, _>(&self.args, "Args must be Array like or Null.")?;
        let kwargs = helpers::ser_value_is_kwargs::<S, _>(&self.kwargs, "Kwargs must be Object like or Null.")?;
        if args.is_null() {
            if kwargs.is_null() {
                (Self::ID, &self.request_id, options, &self.topic).serialize(serializer)
            } else {
                (Self::ID, &self.request_id, options, &self.topic, json!([]), kwargs).serialize(serializer)
            }
        } else {
            if kwargs.is_null() {
                (Self::ID, &self.request_id, options, &self.topic, args).serialize(serializer)
            } else {
                (Self::ID, &self.request_id, options, &self.topic, args, kwargs).serialize(serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for Publish {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct PublishVisitor(PhantomData<u8>, PhantomData<u64>, PhantomData<Value>, PhantomData<String>, PhantomData<Value>, PhantomData<Value>);
        
        impl<'vi> Visitor<'vi> for PublishVisitor {
            type Value = Publish;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Publish components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'vi> {
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message ID must be present and type u8.")?;
                helpers::validate_id::<Publish, A, _>(&message_id, "Publish")?;
                let request_id: u64 = helpers::deser_seq_element(&mut seq, "Request ID must be present and type u64.")?;
                let options: Value = helpers::deser_seq_element(&mut seq, "Options must be present and object like.")?;
                helpers::deser_value_is_object::<A, _>(&options, "Options must be object like.")?;
                let topic: String = helpers::deser_seq_element(&mut seq, "topic must be present and object like.")?;
                let args: Value = helpers::deser_seq_element(&mut seq, "Args must be array like or null.")?;
                let kwargs: Value = helpers::deser_seq_element(&mut seq, "Kwargs must be object like or null.")?;
                Ok(Publish {
                    request_id,
                    options,
                    topic,
                    args,
                    kwargs
                })
            }
        }

        deserializer.deserialize_struct("Publish", &["request_id", "options", "topic", "args", "kwargs"], PublishVisitor(PhantomData, PhantomData, PhantomData, PhantomData, PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, from_str, to_string};

    use super::Publish;


    #[test]
    fn raw_str() {
        let d1 = r#"[48,7814135,{},"com.myapp.user.new",["johnny"],{"firstname":"John","surname":"Doe"}]"#;
        let mut p1 = Publish {
            request_id: 7814135,
            options: json!({}),
            topic: "com.myapp.user.new".to_string(),
            args: serde_json::Value::Null,
            kwargs: json!({"firstname":"John","surname":"Doe"})
        };
        let p2: Publish = from_str(&d1).unwrap();
        
        assert_ne!(p1, p2);
        p1.args = json!(["johnny"]);
        let d2 = to_string(&p1).unwrap();
        assert_eq!(p1, p2);
        assert_eq!(d1, d2);
    }


}