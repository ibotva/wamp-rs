use std::fmt::Formatter;
use std::marker::PhantomData;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{SeqAccess, Visitor};
use crate::protocol::roles::Roles;
use super::{helpers, WampMessage, MessageDirection};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unregistered {
    pub request_id: u64
}

#[macro_export]
macro_rules! unregistered {
    ($request_id:expr) => {
        Unregistered{
            request_id: $request_id
        }
    };
}


impl WampMessage<Unregistered> for Unregistered {
    const ID: u64 = 67;

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

impl Serialize for Unregistered {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        (Self::ID, &self.request_id).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Unregistered {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct UnregisteredVisitor(PhantomData<u64>, PhantomData<u64>, PhantomData<u64>);
        
        impl<'vi> Visitor<'vi> for UnregisteredVisitor {
            type Value = Unregistered;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Unregistered components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'vi> {
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message ID must be present and type u8.")?;
                helpers::validate_id::<Unregistered, A, _>(&message_id, "Unregistered")?;
                let request_id: u64 = helpers::deser_seq_element(&mut seq, "request_id must be present and type u64.")?;
                Ok(Unregistered {
                    request_id
                })
            }
        }

        deserializer.deserialize_struct("Unregistered", &["request_id", "registration"], UnregisteredVisitor(PhantomData, PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::Unregistered;

    #[test]
    fn test() {
        let d1 = r#"[67,788923562]"#;
        let p1 = Unregistered {
            request_id: 788923562
        };
        assert_eq!(d1, to_string(&p1).unwrap());
        assert_eq!(from_str::<Unregistered>(d1).unwrap(), p1);
    }
}