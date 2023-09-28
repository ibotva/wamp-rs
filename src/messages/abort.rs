use std::marker::PhantomData;

use serde::{Serialize, de::{self, Visitor}, Deserialize};
use serde_json::Value;

use super::{WampMessage, helpers};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abort {
    pub details: Value,
    pub reason: String
}

impl WampMessage for Abort {
    const ID: u8 = 3;
}

impl Serialize for Abort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let details = helpers::ser_value_is_object::<S, _>(&self.details, "Details must be object like.")?;
        (Self::ID, &details, &self.reason).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Abort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where 
        D: serde::Deserializer<'de> 
    {
        struct AbortVisitor(PhantomData<u8>, PhantomData<String>, PhantomData<Value>);

        impl<'vi> Visitor<'vi> for AbortVisitor {
            type Value = Abort;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("WAMP Abort frame, expressed as a sequence.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'vi>, 
            {
                let message_id: u8 = helpers::deser_seq_element(&mut seq, "Message ID must be type u8.")?;
                helpers::validate_id::<Abort, A, _>(&message_id, "Abort")?;
                let details: Value = helpers::deser_seq_element(&mut seq, "Details must be a JSON value.")?;
                let reason: String = helpers::deser_seq_element(&mut seq, "Reason must be a String.")?;
                helpers::deser_value_is_object::<A, _>(&details, "Details must be object like.")?;
                Ok(Abort { reason, details })
            }
        }
        
        deserializer.deserialize_struct("Abort", &["reason", "details"], AbortVisitor(PhantomData, PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, json, to_string};

    use super::*;

    #[test]
    fn raw_str() {
        let data = r#"[3,{"message":"The realm does not exist."},"wamp.error.no_such_realm"]"#;
        let a: Abort = from_str(data).unwrap();
        println!("{:#?}", a);
        assert_eq!(a.reason, "wamp.error.no_such_realm");
    }

    #[test]
    fn obj_to_str() {
        let a = Abort {
            details: json!({"message":"The realm does not exist."}),
            reason: "wamp.error.no_such_realm".to_string()
        };
        let data = r#"[3,{"message":"The realm does not exist."},"wamp.error.no_such_realm"]"#;
        let an = to_string(&a).unwrap();
        assert!(data == an, "{data} == {an}")
    }
}