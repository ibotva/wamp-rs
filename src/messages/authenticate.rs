use std::marker::PhantomData;

use serde::{Serialize, Deserialize, de::Visitor};
use serde_json::Value;

use crate::roles::Roles;

use super::{helpers, WampMessage, MessageDirection};

pub struct Authenticate {
    pub signature: String,
    pub details: Value
}

impl WampMessage for Authenticate {
    const ID: u64 = 5;

    fn direction(r: crate::roles::Roles) -> &'static super::MessageDirection {
        match r {
            Roles::Callee => &MessageDirection {
                receives: &false,
                sends: &true,
            },
            Roles::Caller => &MessageDirection {
                receives: &false,
                sends: &true,
            },
            Roles::Publisher => &MessageDirection {
                receives: &false,
                sends: &true,
            },
            Roles::Subscriber => &MessageDirection {
                receives: &false,
                sends: &true,
            },
            Roles::Dealer => &MessageDirection {
                receives: &true,
                sends: &false,
            },
            Roles::Broker => &MessageDirection {
                receives: &true,
                sends: &false,
            },
        }
    }
}

impl Serialize for Authenticate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let details = helpers::ser_value_is_object::<S, _>(&self.details, "Details must be object like.")?;
        (Self::ID, &self.signature, details).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Authenticate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        struct AuthenticateVisitor(PhantomData<u8>, PhantomData<String>, PhantomData<Value>);

        impl<'vi> Visitor<'vi> for AuthenticateVisitor {
            type Value = Authenticate;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Wamp message containing authentication details")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::SeqAccess<'vi>
            {
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message ID must be present and type u8.")?;
                helpers::validate_id::<Authenticate, A, _>(&message_id, "Authenticate")?;
                let signature: String = helpers::deser_seq_element(&mut seq, "Signature must be type String.")?;
                let details: Value = helpers::deser_seq_element(&mut seq, "Details must be present and object like.")?;
                helpers::deser_value_is_object::<A, _>(&details, "Value must be object like")?;
                Ok(Authenticate { signature, details })
            }
        }

        deserializer.deserialize_struct("Authenticate", &["signature", "details"], AuthenticateVisitor(PhantomData, PhantomData, PhantomData))
    }
}