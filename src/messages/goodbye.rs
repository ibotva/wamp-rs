use std::marker::PhantomData;

use serde::{Serialize, de::{self, Visitor}, Deserialize};
use serde_json::Value;

use super::{WampMessage, helpers};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Goodbye {
    pub details: Value,
    pub reason: String
}

impl WampMessage for Goodbye {
    const ID: u64 = 6;
}

impl Serialize for Goodbye {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let details = helpers::ser_value_is_object::<S, _>(&self.details, "Details must be object like.")?;
        (Self::ID, &details, &self.reason).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Goodbye {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where 
        D: serde::Deserializer<'de> 
    {
        struct GoodbyeVisitor(PhantomData<u8>, PhantomData<String>, PhantomData<Value>);

        impl<'vi> Visitor<'vi> for GoodbyeVisitor {
            type Value = Goodbye;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("WAMP Goodbye frame, expressed as a sequence.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'vi>, 
            {
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message ID must be type u8.")?;
                helpers::validate_id::<Goodbye, A, _>(&message_id, "Goodbye")?;
                let details: Value = helpers::deser_seq_element(&mut seq, "Details must be a JSON value.")?;
                let reason: String = helpers::deser_seq_element(&mut seq, "Reason must be a String.")?;
                helpers::deser_value_is_object::<A, _>(&details, "Details must be object like.")?;
                Ok(Goodbye { reason, details })
            }
        }
        
        deserializer.deserialize_struct("Goodbye", &["reason", "details"], GoodbyeVisitor(PhantomData, PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{to_string, from_str};

    use super::Goodbye;

    #[test]
    fn test() {
        let d1 = r#"[6,{"message":"The host is shutting down now."},"wamp.close.system_shutdown"]"#;
        let g1 = Goodbye {
            details: serde_json::json!({"message":"The host is shutting down now."}),
            reason: "wamp.close.system_shutdown".to_string()
        };
        let d2 = to_string(&g1).unwrap();
        let g2: Goodbye = from_str(d1).unwrap();
        assert_eq!(d1, d2);
        assert_eq!(g1, g2);
    }
}