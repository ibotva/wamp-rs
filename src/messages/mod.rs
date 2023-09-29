mod abort;
mod call;
mod authenticate;
mod cancel;
mod challenge;
mod error;
mod event;
mod goodbye;
mod hello;
mod interrupt;
mod invocation;
mod publish;
mod published;
mod register;
mod registered;
mod result;
mod subscribe;
mod subscribed;
mod unregister;
mod unregistered;
mod unsubscribe;
mod unsubscribed;
mod welcome;
mod r#yield;

pub use abort::Abort;

use crate::roles::Roles;

pub(crate) mod helpers {

    use std::fmt::Display;
    use serde::{de::{SeqAccess, self}, Deserialize, Serializer, ser::Error};
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

    pub(crate) fn validate_id<'de, T: WampMessage, A: SeqAccess<'de>, E: Display>(id: &u64, name: E) -> Result<(), A::Error> {
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

    pub(crate) fn ser_value_is_args<S: Serializer, T: Display>(v: &Value, e: T) -> Result<&Value, S::Error> {
        if v.is_array() || v.is_null() {
            Ok(v)
        } else {
            Err(S::Error::custom(e))
        }
    }

    pub(crate) fn ser_value_is_kwargs<S: Serializer, T: Display>(v: &Value, e: T) -> Result<&Value, S::Error> {
        if v.is_object() || v.is_null() {
            Ok(v)
        } else {
            Err(S::Error::custom(e))
        }
    }


}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct MessageDirection {
    pub receives: &'static bool,
    pub sends: &'static bool,
}

pub trait WampMessage {
    const ID: u64;

    fn direction(role: Roles) -> &'static MessageDirection;
}


