use std::marker::PhantomData;

use serde_json::Value;
use serde::{Serialize, Deserialize, de::Visitor};

use crate::messages::helpers;

use super::WampMessage;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Register  {
    pub request_id: u64,
    pub options: Value,
    pub procedure: String,
}

impl WampMessage for Register {
    const ID: u64 = 64;
}

impl Serialize for Register {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        (Self::ID, &self.request_id, &self.options, &self.procedure).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Register {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        struct RegisterVisitor(PhantomData<u64>, PhantomData<u64>, PhantomData<Value>, PhantomData<String>);

        impl<'vi> Visitor<'vi> for RegisterVisitor {
            type Value = Register;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Register components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                 A: serde::de::SeqAccess<'vi>, 
            {   
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message id must be present and type u64.")?;
                helpers::validate_id::<Register, A, _>(&message_id, "Register")?;
                let request_id: u64 = helpers::deser_seq_element(&mut seq, "Request ID must be present and type u64")?;
                let options: Value = helpers::deser_seq_element(&mut seq, "options must be present and object like")?;
                helpers::deser_value_is_object::<A, _>(&options, "options must be object like.")?;
                let procedure: String = helpers::deser_seq_element(&mut seq, "procedure URI must be present and type String")?;
                helpers::deser_value_is_object::<A, _>(&options, "options must be object like.")?;
                Ok(Register {
                    request_id,
                    options,
                    procedure
                })
            }
        }

        deserializer.deserialize_struct("Register", &["request_id", "options", "procedure"], RegisterVisitor(PhantomData, PhantomData, PhantomData, PhantomData))


    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string, json};

    use super::Register;

    #[test]
    fn register_test() {
        let d1 = r#"[64,25349185,{},"com.myapp.myprocedure1"]"#;
        let r1 = Register {
            request_id: 25349185,
            options: json!({}),
            procedure: "com.myapp.myprocedure1".to_string()
        };
        assert_eq!(d1, to_string(&r1).unwrap());
        assert_eq!(r1, from_str::<Register>(d1).unwrap())
    }
}