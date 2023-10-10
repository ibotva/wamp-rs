use std::marker::PhantomData;
use serde_json::Value;
use serde::{Serialize, Deserialize, de::Visitor};
use crate::protocol::{messages::helpers, roles::Roles};
use super::{WampMessage, MessageDirection};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subscribe  {
    pub request_id: u64,
    pub options: Value,
    pub topic: String,
}

#[macro_export]
macro_rules! subscribe {
    ($topic:expr) => {
        core::subscribe!{$topic, serde_json::json!({})}
    };
    ($topic:expr, $options:expr) => {
        $crate::protocol::messages::Subscribe {
            topic: $topic.to_string(),
            options: $options,
            request_id: $crate::protocol::increment()
        }
    };
}

impl WampMessage<Subscribe> for Subscribe {
    const ID: u64 = 32;

    fn direction(role: Roles) -> &'static MessageDirection {
        match role {
            Roles::Callee => &MessageDirection {
                receives: &false,
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
                sends: &true,
            },
            Roles::Dealer => &MessageDirection {
                receives: &false,
                sends: &false,
            },
            Roles::Broker => &MessageDirection {
                receives: &true,
                sends: &false,
            },
        }
    }
}

impl Serialize for Subscribe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        (Self::ID, &self.request_id, &self.options, &self.topic).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Subscribe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        struct SubscribeVisitor(PhantomData<u64>, PhantomData<u64>, PhantomData<Value>, PhantomData<String>);

        impl<'vi> Visitor<'vi> for SubscribeVisitor {
            type Value = Subscribe;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Subscribe components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                 A: serde::de::SeqAccess<'vi>, 
            {   
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message id must be present and type u64.")?;
                helpers::validate_id::<Subscribe, A, _>(&message_id, "Subscribe")?;
                let request_id: u64 = helpers::deser_seq_element(&mut seq, "Request ID must be present and type u64")?;
                let options: Value = helpers::deser_seq_element(&mut seq, "options must be present and object like")?;
                helpers::deser_value_is_object::<A, _>(&options, "options must be object like.")?;
                let topic: String = helpers::deser_seq_element(&mut seq, "topic URI must be present and type String")?;
                helpers::deser_value_is_object::<A, _>(&options, "options must be object like.")?;
                Ok(Subscribe {
                    request_id,
                    options,
                    topic
                })
            }
        }

        deserializer.deserialize_struct("Subscribe", &["request_id", "options", "topic"], SubscribeVisitor(PhantomData, PhantomData, PhantomData, PhantomData))


    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string, json};

    use super::Subscribe;

    #[test]
    fn subscribe_test() {
        let d1 = r#"[32,713845233,{},"com.myapp.mytopic1"]"#;
        let r1 = Subscribe {
            request_id: 713845233,
            options: json!({}),
            topic: "com.myapp.mytopic1".to_string()
        };
        assert_eq!(d1, to_string(&r1).unwrap());
        assert_eq!(r1, from_str::<Subscribe>(d1).unwrap())
    }
}