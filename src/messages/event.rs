use std::fmt::Formatter;
use std::marker::PhantomData;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{SeqAccess, Visitor};
use serde_json::{json, Value};

use crate::roles::Roles;

use super::{helpers, WampMessage, MessageDirection};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub subscription: u64,
    pub publication: u64,
    pub details: Value,
    pub args: Value,
    pub kwargs: Value
}

impl WampMessage for Event {
    const ID: u64 = 36;

    fn direction(r: Roles) -> &'static MessageDirection {
        match r {
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
                receives: &true,
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

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        
        let args = helpers::ser_value_is_args::<S, _>(&self.args, "Args must be Array like or Null.")?;
        let kwargs = helpers::ser_value_is_kwargs::<S, _>(&self.kwargs, "Kwargs must be Object like or Null.")?;

        if args.is_null() {
            if kwargs.is_null() {

            }
        } else {
            if kwargs.is_null() {

            }
        }

        if args.is_null() {
            if kwargs.is_null() {
                (Self::ID, &self.subscription, &self.publication, &self.details).serialize(serializer)
            } else {
                (Self::ID, &self.subscription, &self.publication, &self.details, json!([]), kwargs).serialize(serializer)
            }
        } else {
            if kwargs.is_null() {
                (Self::ID, &self.subscription, &self.publication, &self.details, args).serialize(serializer)
            } else {
                (Self::ID, &self.subscription, &self.publication, &self.details, args, kwargs).serialize(serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct EventVisitor(PhantomData<u8>, PhantomData<u64>, PhantomData<u64>, PhantomData<Value>, PhantomData<Value>, PhantomData<Value>);
        
        impl<'vi> Visitor<'vi> for EventVisitor {
            type Value = Event;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Event components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'vi> {
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message ID must be present and type u8.")?;
                helpers::validate_id::<Event, A, _>(&message_id, "Event")?;
                let subscription: u64 = helpers::deser_seq_element(&mut seq, "Subscription must be present and type u64.")?;
                let publication: u64 = helpers::deser_seq_element(&mut seq, "Publication must be present and object like.")?;
                let details: Value = helpers::deser_seq_element(&mut seq, "Details must be present and object like.")?;
                helpers::deser_value_is_object::<A, _>(&details, "Details must be object like.")?;
                let args: Value = helpers::deser_seq_element(&mut seq, "Args must be array like or null.")?;
                let kwargs: Value = helpers::deser_seq_element(&mut seq, "Kwargs must be object like or null.")?;
                Ok(Event {
                    subscription,
                    publication,
                    details,
                    args,
                    kwargs
                })
            }
        }

        deserializer.deserialize_struct("Event", &["subscription", "publication", "details", "args", "kwargs"], EventVisitor(PhantomData, PhantomData, PhantomData, PhantomData, PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::Event;

    #[test]
    fn test() {
        let d = r#"[36,5512315355,4429313566,{},[],{"color":"orange","sizes":[23,42,7]}]"#;
        let mut ed = Event {
            subscription: 5512315355,
            publication: 4429313566,
            details: serde_json::json!({}),
            args: serde_json::Value::Null,
            kwargs: serde_json::json!({"color":"orange","sizes":[23,42,7]})
        };
        let ed2: Event = from_str(d).unwrap();
        let d2 = to_string(&ed).unwrap();
        assert_ne!(ed, ed2);
        ed.args = serde_json::json!([]);
        assert_eq!(ed, ed2);
        assert_eq!(d, d2);
    }
}