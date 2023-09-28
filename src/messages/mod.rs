mod abort;

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

    pub(crate) fn validate_id<'de, T: WampMessage, D: Deserializer<'de>, E: Display>(id: &u8, name: E) -> Result<(), D::Error> {
        if &T::ID == id {
            Ok(())
        } else {
            Err(de::Error::custom(name))
        }
    }

    pub(crate) fn value_is_object<S: Serializer, T: Display>(v: &Value, e: T) -> Result<&Value, S::Error> {
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

