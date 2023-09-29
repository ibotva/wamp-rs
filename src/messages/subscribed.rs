use std::fmt::Formatter;
use std::marker::PhantomData;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{SeqAccess, Visitor};

use super::{helpers, WampMessage};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subscribed {
    pub request_id: u64,
    pub subscription: u64,
}

impl WampMessage for Subscribed {
    const ID: u64 = 33;
}

impl Serialize for Subscribed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        (Self::ID, &self.request_id, &self.subscription).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Subscribed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct SubscribedVisitor(PhantomData<u64>, PhantomData<u64>, PhantomData<u64>);
        
        impl<'vi> Visitor<'vi> for SubscribedVisitor {
            type Value = Subscribed;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Subscribed components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'vi> {
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message ID must be present and type u8.")?;
                helpers::validate_id::<Subscribed, A, _>(&message_id, "Subscribed")?;
                let request_id: u64 = helpers::deser_seq_element(&mut seq, "request_id must be present and type u64.")?;
                let subscription: u64 = helpers::deser_seq_element(&mut seq, "subscription must be present and object like.")?;
                Ok(Subscribed {
                    request_id,
                    subscription
                })
            }
        }

        deserializer.deserialize_struct("Subscribed", &["request_id", "subscription"], SubscribedVisitor(PhantomData, PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::Subscribed;

    #[test]
    fn test() {
        let d1 = r#"[33,713845233,5512315355]"#;
        let p1 = Subscribed {
            request_id: 713845233,
            subscription: 5512315355
        };
        assert_eq!(d1, to_string(&p1).unwrap());
        assert_eq!(from_str::<Subscribed>(d1).unwrap(), p1);
    }
}