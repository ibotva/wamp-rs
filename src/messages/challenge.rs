use std::marker::PhantomData;

use serde::{Serialize, Deserialize, de::Visitor};
use serde_json::Value;

use crate::roles::Roles;

use super::{helpers, WampMessage, MessageDirection};

#[derive(Debug)]
pub struct Challenge {
    pub authmethod: String,
    pub details: Value
}

impl WampMessage for Challenge {
    const ID: u64 = 4;

    fn direction(role: Roles) -> &'static MessageDirection {
        match role {
            Roles::Callee => &MessageDirection {
                receives: &true,
                sends: &false,
            },
            Roles::Caller => &MessageDirection {
                receives: &true,
                sends: &false,
            },
            Roles::Publisher => &MessageDirection {
                receives: &true,
                sends: &false,
            },
            Roles::Subscriber => &MessageDirection {
                receives: &true,
                sends: &false,
            },
            Roles::Dealer => &MessageDirection {
                receives: &false,
                sends: &true,
            },
            Roles::Broker => &MessageDirection {
                receives: &false,
                sends: &true,
            },
        }
    }
}

impl Serialize for Challenge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let details = helpers::ser_value_is_object::<S, _>(&self.details, "Details must be object like.")?;
        (Self::ID, &self.authmethod, details).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Challenge {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        struct ChallengeVisitor(PhantomData<u8>, PhantomData<String>, PhantomData<Value>);

        impl<'vi> Visitor<'vi> for ChallengeVisitor {
            type Value = Challenge;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Wamp message containing authentication details")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::SeqAccess<'vi>
            {
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message ID must be present and type u8.")?;
                helpers::validate_id::<Challenge, A, _>(&message_id, "Challenge")?;
                let authmethod: String = helpers::deser_seq_element(&mut seq, "authmethod must be type String.")?;
                let details: Value = helpers::deser_seq_element(&mut seq, "Details must be present and object like.")?;
                helpers::deser_value_is_object::<A, _>(&details, "Value must be object like")?;
                Ok(Challenge { authmethod, details })
            }
        }

        deserializer.deserialize_struct("Challenge", &["authmethod", "details"], ChallengeVisitor(PhantomData, PhantomData, PhantomData))
    }
}


#[cfg(test)]
mod tests {
    use serde_json::{json, to_string};

    use super::*;

    #[test]
    fn obj_to_str() {
        let a = Challenge {
            authmethod: "wampcra".to_string(),
            details: json!({
                "challenge": {
                    "authid":"peter",
                    "authmethod":"wampcra",
                    "authprovider":"userdb",
                    "authrole":"user",
                    "nonce": "LHRTC9zeOIrt_9U3",
                    "session": 3251278072152162 as u64,
                    "timestamp":"2014-06-22T16:36:25.448Z",
                }    
            }),
        };
        let data = r#"[4,"wampcra",{"challenge":{"authid":"peter","authmethod":"wampcra","authprovider":"userdb","authrole":"user","nonce":"LHRTC9zeOIrt_9U3","session":3251278072152162,"timestamp":"2014-06-22T16:36:25.448Z"}}]"#;
        let an = to_string(&a).unwrap();
        assert_eq!(data, an)
    }
}
