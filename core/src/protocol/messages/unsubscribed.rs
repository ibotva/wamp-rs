use std::fmt::Formatter;
use std::marker::PhantomData;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{SeqAccess, Visitor};
use crate::protocol::roles::Roles;
use super::{helpers, WampMessage, MessageDirection};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unsubscribed {
    pub request_id: u64
}

// Commented out because WAMP spec requires the return ID to be the same as the request, and as this just gets the statically 
// Incremented mutable variable, it does not follow spec.
#[macro_export]
macro_rules! unsubscribed {
    ($request_id:expr) => {
        Unsubscribed{
            request_id: $request_id
        }
    };
}

impl WampMessage<Unsubscribed> for Unsubscribed {
    const ID: u64 = 35;

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

impl Serialize for Unsubscribed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        (Self::ID, &self.request_id).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Unsubscribed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct UnsubscribedVisitor(PhantomData<u64>, PhantomData<u64>, PhantomData<u64>);
        
        impl<'vi> Visitor<'vi> for UnsubscribedVisitor {
            type Value = Unsubscribed;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Unsubscribed components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'vi> {
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message ID must be present and type u8.")?;
                helpers::validate_id::<Unsubscribed, A, _>(&message_id, "Unsubscribed")?;
                let request_id: u64 = helpers::deser_seq_element(&mut seq, "request_id must be present and type u64.")?;
                Ok(Unsubscribed {
                    request_id
                })
            }
        }

        deserializer.deserialize_struct("Unsubscribed", &["request_id", "registration"], UnsubscribedVisitor(PhantomData, PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::Unsubscribed;

    #[test]
    fn test() {
        let d1 = r#"[35,85346237]"#;
        let p1 = Unsubscribed {
            request_id: 85346237
        };
        assert_eq!(d1, to_string(&p1).unwrap());
        assert_eq!(from_str::<Unsubscribed>(d1).unwrap(), p1);
    }
}