use std::marker::PhantomData;

use serde::{Serialize, de::{self, Visitor}, Deserialize};
use serde_json::Value;

use crate::roles::Roles;

use super::{WampMessage, helpers, MessageDirection};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interrupt {
    pub request_id: u64,
    pub options: Value
}

impl WampMessage for Interrupt {
    const ID: u64 = 69;

    fn direction(r: Roles) -> &'static MessageDirection {
        match r {
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

impl Serialize for Interrupt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let options = helpers::ser_value_is_object::<S, _>(&self.options, "Options must be object like.")?;
        (Self::ID, &self.request_id, options).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Interrupt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where 
        D: serde::Deserializer<'de> 
    {
        struct InterruptVisitor(PhantomData<u8>, PhantomData<String>, PhantomData<Value>);

        impl<'vi> Visitor<'vi> for InterruptVisitor {
            type Value = Interrupt;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("WAMP Interrupt frame, expressed as a sequence.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'vi>, 
            {
                let message_id: u64 = helpers::deser_seq_element(&mut seq, "Message ID must be type u64.")?;
                helpers::validate_id::<Interrupt, A, _>(&message_id, "Interrupt")?;
                let request_id: u64 = helpers::deser_seq_element(&mut seq, "Request ID must be a u64.")?;
                let options: Value = helpers::deser_seq_element(&mut seq, "Options must be a JSON value.")?;
                helpers::deser_value_is_object::<A, _>(&options, "Options must be object like.")?;
                Ok(Interrupt { request_id, options })
            }
        }
        
        deserializer.deserialize_struct("Interrupt", &["request_id", "options"], InterruptVisitor(PhantomData, PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{to_string, from_str};

    use crate::messages::interrupt::Interrupt;


    #[test]
    fn test() {
        let d1 = r#"[69,3,{}]"#;
        let g1 = Interrupt {
            options: serde_json::json!({}),
            request_id: 3
        };
        let d2 = to_string(&g1).unwrap();
        assert_eq!(d1, d2);
        let g2: Interrupt = from_str(d1).unwrap();
        assert_eq!(g1, g2);
    }
}