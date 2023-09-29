use std::marker::PhantomData;

use serde::{Serialize, de::{self, Visitor}, Deserialize};
use serde_json::Value;

use super::{WampMessage, helpers};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hello {
    pub realm: String,
    pub details: Value
}

impl WampMessage for Hello {
    const ID: u64 = 1;
}

impl Serialize for Hello {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let details = helpers::ser_value_is_object::<S, _>(&self.details, "Details must be object like.")?;
        (Self::ID, &self.realm, &details).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Hello {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where 
        D: serde::Deserializer<'de> 
    {
        struct HelloVisitor(PhantomData<u8>, PhantomData<String>, PhantomData<Value>);

        impl<'vi> Visitor<'vi> for HelloVisitor {
            type Value = Hello;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("WAMP Hello frame, expressed as a sequence.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'vi>, 
            {
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message ID must be type u8.")?;
                helpers::validate_id::<Hello, A, _>(&message_id, "Hello")?;
                let realm: String = helpers::deser_seq_element(&mut seq, "realm must be a String.")?;
                let details: Value = helpers::deser_seq_element(&mut seq, "Details must be a JSON value.")?;
                helpers::deser_value_is_object::<A, _>(&details, "Details must be object like.")?;
                Ok(Hello { realm, details })
            }
        }
        
        deserializer.deserialize_struct("Hello", &["realm", "details"], HelloVisitor(PhantomData, PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{to_string, from_str};

    use super::Hello;

    #[test]
    fn test() {
        let d1 = r#"[1,"somerealm",{"roles":{"publisher":{},"subscriber":{}}}]"#;
        let g1 = Hello {
            details: serde_json::json!({"roles":{"publisher":{},"subscriber":{}}}),
            realm: "somerealm".to_string()
        };
        let d2 = to_string(&g1).unwrap();
        let g2: Hello = from_str(d1).unwrap();
        assert_eq!(d1, d2);
        assert_eq!(g1, g2);
    }
}