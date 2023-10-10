use std::fmt::Formatter;
use std::marker::PhantomData;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{SeqAccess, Visitor};
use crate::protocol::roles::Roles;
use super::{helpers, WampMessage, MessageDirection};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Registered {
    pub request_id: u64,
    pub registration: u64,
}

#[macro_export]
macro_rules! registered {
    ($request_id:expr, $registration:expr) => {
        Registered {
            request_id: $request_id,
            registration: $registration
        }
    };
}

impl WampMessage<Registered> for Registered {
    const ID: u64 = 65;

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

impl Serialize for Registered {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        (Self::ID, &self.request_id, &self.registration).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Registered {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct RegisteredVisitor(PhantomData<u64>, PhantomData<u64>, PhantomData<u64>);
        
        impl<'vi> Visitor<'vi> for RegisteredVisitor {
            type Value = Registered;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Registered components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'vi> {
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message ID must be present and type u8.")?;
                helpers::validate_id::<Registered, A, _>(&message_id, "Registered")?;
                let request_id: u64 = helpers::deser_seq_element(&mut seq, "request_id must be present and type u64.")?;
                let registration: u64 = helpers::deser_seq_element(&mut seq, "registration must be present and object like.")?;
                Ok(Registered {
                    request_id,
                    registration
                })
            }
        }

        deserializer.deserialize_struct("Registered", &["request_id", "registration"], RegisteredVisitor(PhantomData, PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::Registered;

    #[test]
    fn test() {
        let d1 = r#"[65,25349185,2103333224]"#;
        let p1 = Registered {
            request_id: 25349185,
            registration: 2103333224
        };
        assert_eq!(d1, to_string(&p1).unwrap());
        assert_eq!(from_str::<Registered>(d1).unwrap(), p1);
    }
}