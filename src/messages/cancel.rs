use std::marker::PhantomData;

use serde::{Serialize, de::{self, Visitor}, Deserialize};
use serde_json::Value;

use super::{WampMessage, helpers};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cancel {
    pub request_id: u64,
    pub options: Value
}

impl WampMessage for Cancel {
    const ID: u64 = 49;
}

impl Serialize for Cancel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let options = helpers::ser_value_is_object::<S, _>(&self.options, "Options must be object like.")?;
        (Self::ID, &self.request_id, options).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Cancel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where 
        D: serde::Deserializer<'de> 
    {
        struct CancelVisitor(PhantomData<u8>, PhantomData<String>, PhantomData<Value>);

        impl<'vi> Visitor<'vi> for CancelVisitor {
            type Value = Cancel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("WAMP Cancel frame, expressed as a sequence.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'vi>, 
            {
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message ID must be type u64.")?;
                helpers::validate_id::<Cancel, A, _>(&message_id, "Cancel")?;
                let request_id: u64 = helpers::deser_seq_element(&mut seq, "Request ID must be a u64.")?;
                let options: Value = helpers::deser_seq_element(&mut seq, "Options must be a JSON value.")?;
                helpers::deser_value_is_object::<A, _>(&options, "Options must be object like.")?;
                Ok(Cancel { request_id, options })
            }
        }
        
        deserializer.deserialize_struct("Cancel", &["request_id", "options"], CancelVisitor(PhantomData, PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, json, to_string};

    use super::*;

    #[test]
    fn raw_str() {
        let data = r#"[3,{"message":"The realm does not exist."},"wamp.error.no_such_realm"]"#;
        let a: Cancel = from_str(data).unwrap();
        println!("{:#?}", a);
        assert_eq!(a.options, "wamp.error.no_such_realm");
    }

    #[test]
    fn obj_to_str() {
        let a = Cancel {
            options: json!({"message":"The realm does not exist."}),
            request_id:1
        };
        let data = r#"[3,{"message":"The realm does not exist."},"wamp.error.no_such_realm"]"#;
        let an = to_string(&a).unwrap();
        assert_eq!(data, an)
    }
}