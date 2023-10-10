use std::fmt::Formatter;
use std::marker::PhantomData;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{SeqAccess, Visitor};
use crate::protocol::roles::Roles;
use super::{helpers, WampMessage, MessageDirection};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unregister {
    pub request_id: u64,
    pub registration: u64,
}

#[macro_export]
macro_rules! unregister {
    ($registration:expr) => {
        Unregister {
            request_id: $crate::protocol::increment(),
            registration: $registration
        }
    };
}

impl WampMessage<Unregister> for Unregister {
    const ID: u64 = 66;

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

impl Serialize for Unregister {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        (Self::ID, &self.request_id, &self.registration).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Unregister {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct UnregisterVisitor(PhantomData<u64>, PhantomData<u64>, PhantomData<u64>);
        
        impl<'vi> Visitor<'vi> for UnregisterVisitor {
            type Value = Unregister;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Unregister components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'vi> {
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message ID must be present and type u8.")?;
                helpers::validate_id::<Unregister, A, _>(&message_id, "Unregister")?;
                let request_id: u64 = helpers::deser_seq_element(&mut seq, "request_id must be present and type u64.")?;
                let registration: u64 = helpers::deser_seq_element(&mut seq, "registration must be present and object like.")?;
                Ok(Unregister {
                    request_id,
                    registration
                })
            }
        }

        deserializer.deserialize_struct("Unregister", &["request_id", "registration"], UnregisterVisitor(PhantomData, PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::Unregister;

    #[test]
    fn test() {
        let d1 = r#"[66,788923562,2103333224]"#;
        let p1 = Unregister {
            request_id: 788923562,
            registration: 2103333224
        };
        assert_eq!(d1, to_string(&p1).unwrap());
        assert_eq!(from_str::<Unregister>(d1).unwrap(), p1);
    }
}