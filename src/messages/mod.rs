mod abort;

pub use abort::Abort;

pub(crate) mod helpers {

    use std::fmt::Display;
    use serde::{de::{SeqAccess, self}, Deserialize, Serializer, ser, ser::Error, Deserializer};
    use serde_json::Value;

    use super::WampMessage;

    pub(crate) fn deser_seq_element<'de, T: PartialEq + Deserialize<'de>, E: Display, A: SeqAccess<'de>>(seq: &mut A, error: E) -> Result<T, <A as SeqAccess<'de>>::Error> {
        let element: Option<T> = seq.next_element()?;
        if element != None {
            Ok(element.unwrap())
        } else {
            Err(serde::de::Error::custom(error))
        }
    }

    pub(crate) fn validate_id<'de, T: WampMessage, A: SeqAccess<'de>, E: Display>(id: &u8, name: E) -> Result<(), A::Error> {
        if &T::ID == id {
            Ok(())
        } else {
            Err(de::Error::custom(format!("{name} has invalid ID {id}. The ID for {name} must be {}", T::ID)))
        }
    }

    pub(crate) fn deser_value_is_object<'de, A: SeqAccess<'de>, E: Display>(v: &Value, e: E) -> Result<(), A::Error>  {
        if v.is_object() {
            Ok(())
        } else {
            Err(de::Error::custom(e))
        }
    }

    pub(crate) fn ser_value_is_object<S: Serializer, T: Display>(v: &Value, e: T) -> Result<&Value, S::Error> {
        if v.is_object() {
            Ok(v)
        } else {
            Err(S::Error::custom(e))
        }
    }

}

pub trait WampMessage {
    const ID: u8;
}

