use std::marker::PhantomData;

use serde::{Serialize, de::{self, Visitor}, Deserialize};
use serde_json::Value;

use super::{WampMessage, helpers};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Welcome {
    pub session: u64,
    pub details: Value
}

impl WampMessage for Welcome {
    const ID: u64 = 2;
}

impl Serialize for Welcome {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let details = helpers::ser_value_is_object::<S, _>(&self.details, "details must be object like.")?;
        (Self::ID, &self.session, details).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Welcome {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where 
        D: serde::Deserializer<'de> 
    {
        struct WelcomeVisitor(PhantomData<u8>, PhantomData<String>, PhantomData<Value>);

        impl<'vi> Visitor<'vi> for WelcomeVisitor {
            type Value = Welcome;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("WAMP Welcome frame, expressed as a sequence.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'vi>, 
            {
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message ID must be type u64.")?;
                helpers::validate_id::<Welcome, A, _>(&message_id, "Welcome")?;
                let session: u64 = helpers::deser_seq_element(&mut seq, "Request ID must be a u64.")?;
                let details: Value = helpers::deser_seq_element(&mut seq, "details must be a JSON value.")?;
                helpers::deser_value_is_object::<A, _>(&details, "details must be object like.")?;
                Ok(Welcome { session, details })
            }
        }
        
        deserializer.deserialize_struct("Welcome", &["session", "details"], WelcomeVisitor(PhantomData, PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, json, to_string};

    use super::*;

    #[test]
    fn test() {
        let d1 = r#"[2,9129137332,{"roles":{"broker":{}}}]"#;
        let w1 = Welcome {
            session: 9129137332,
            details: json!({"roles": {
                "broker": {}
            }})
        };
        assert_eq!(w1, from_str(d1).unwrap());
        assert_eq!(d1, to_string(&w1).unwrap());
    }

}