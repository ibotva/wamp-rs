use std::fmt::Formatter;
use std::marker::PhantomData;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{SeqAccess, Visitor};

use crate::roles::Roles;

use super::{helpers, WampMessage, MessageDirection};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Published {
    pub request_id: u64,
    pub publication: u64,
}

impl WampMessage for Published {
    const ID: u64 = 17;

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
                receives: &true,
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

impl Serialize for Published {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        (Self::ID, &self.request_id, &self.publication).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Published {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct PublishedVisitor(PhantomData<u64>, PhantomData<u64>, PhantomData<u64>);
        
        impl<'vi> Visitor<'vi> for PublishedVisitor {
            type Value = Published;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Published components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'vi> {
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message ID must be present and type u8.")?;
                helpers::validate_id::<Published, A, _>(&message_id, "Published")?;
                let request_id: u64 = helpers::deser_seq_element(&mut seq, "request_id must be present and type u64.")?;
                let publication: u64 = helpers::deser_seq_element(&mut seq, "publication must be present and object like.")?;
                Ok(Published {
                    request_id,
                    publication
                })
            }
        }

        deserializer.deserialize_struct("Published", &["request_id", "publication"], PublishedVisitor(PhantomData, PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::Published;

    #[test]
    fn test() {
        let d1 = r#"[17,239714735,4429313566]"#;
        let p1 = Published {
            request_id: 239714735,
            publication: 4429313566
        };
        assert_eq!(d1, to_string(&p1).unwrap());
        assert_eq!(from_str::<Published>(d1).unwrap(), p1);
    }
}