pub mod abort;
pub mod call;
pub mod authenticate;
pub mod cancel;
pub mod challenge;
pub mod error;
pub mod event;
pub mod goodbye;
pub mod hello;
pub mod interrupt;
pub mod invocation;
pub mod publish;
pub mod published;
pub mod register;
pub mod registered;
pub mod result;
pub mod subscribe;
pub mod subscribed;
pub mod unregister;
pub mod unregistered;
pub mod unsubscribe;
pub mod unsubscribed;
pub mod welcome;
pub mod r#yield;

pub use abort::Abort;
pub use call::Call;
pub use authenticate::Authenticate;
pub use cancel::Cancel;
pub use challenge::Challenge;
pub use error::{WampError, WampErrorEvent};
pub use event::Event;
pub use goodbye::Goodbye;
pub use hello::Hello;
pub use interrupt::Interrupt;
pub use invocation::Invocation;
pub use publish::Publish;
pub use published::Published;
pub use register::Register;
pub use registered::Registered;
pub use result::WampResult;
pub use subscribe::Subscribe;
pub use subscribed::Subscribed;
pub use unregister::Unregister;
pub use unregistered::Unregistered;
pub use unsubscribe::Unsubscribe;
pub use unsubscribed::Unsubscribed;
pub use welcome::Welcome;
pub use r#yield::Yield;

use serde::{Deserialize, de, Deserializer};
use serde_json::{Value, json, from_value};

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

pub enum Messages {
    Abort(Abort),
    Authenticate(Authenticate),
    Call(Call),
    Cancel(Cancel),
    Challenge(Challenge),
    Error(WampError),
    Event(Event),
    Goodbye(Goodbye),
    Hello(Hello),
    Interrupt(Interrupt),
    Invocation(Invocation),
    Publish(Publish),
    Published(Published),
    Register(Register),
    Registered(Registered),
    Result(WampResult),
    Subscribe(Subscribe),
    Subscribed(Subscribed),
    Unregister(Unregister),
    Unregistered(Unregistered),
    Unsubscribe(Unsubscribe),
    Unsubscribed(Unsubscribed),
    Welcome(Welcome),
    Yield(Yield),
    Extension(Vec<Value>)
}

impl<'de> Deserialize<'de> for Messages {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        let wamp_components: Vec<Value> = Deserialize::deserialize(deserializer)?;
        let wamp_message_id = match wamp_components.first() {
            Some(v) => { 
                match v.as_u64() {
                    Some(v) => Ok(v),
                    None => Err(de::Error::custom(""))
                }
            },
            None => {
                Err(de::Error::custom("value"))
            }
        }?;

        fn helper<'d, T, D>(wamp_components: Vec<Value>) -> Result<T, D::Error>
        where
            T: for<'de> Deserialize<'de>,
            D: Deserializer<'d>
        {
            let value: T = from_value(json!(wamp_components))
                .map_err(de::Error::custom)?;
            Ok(value)
        }

        match wamp_message_id {
            Abort::ID => Ok(Self::Abort(helper::<Abort, D>(wamp_components)?)),
            Authenticate::ID => Ok(Self::Authenticate(helper::<Authenticate, D>(wamp_components)?)),
            Call::ID => Ok(Self::Call(helper::<Call, D>(wamp_components)?)),
            Cancel::ID => Ok(Self::Cancel(helper::<Cancel, D>(wamp_components)?)),
            Challenge::ID => Ok(Self::Challenge(helper::<Challenge, D>(wamp_components)?)),
            WampError::ID => Ok(Self::Error(helper::<WampError, D>(wamp_components)?)),
            Event::ID => Ok(Self::Event(helper::<Event, D>(wamp_components)?)),
            Goodbye::ID => Ok(Self::Goodbye(helper::<Goodbye, D>(wamp_components)?)),
            Hello::ID => Ok(Self::Hello(helper::<Hello, D>(wamp_components)?)),
            Interrupt::ID => Ok(Self::Interrupt(helper::<Interrupt, D>(wamp_components)?)),
            Invocation::ID => Ok(Self::Invocation(helper::<Invocation, D>(wamp_components)?)),
            Publish::ID => Ok(Self::Publish(helper::<Publish, D>(wamp_components)?)),
            Published::ID => Ok(Self::Published(helper::<Published, D>(wamp_components)?)),
            Register::ID => Ok(Self::Register(helper::<Register, D>(wamp_components)?)),
            Registered::ID => Ok(Self::Registered(helper::<Registered, D>(wamp_components)?)),
            WampResult::ID => Ok(Self::Result(helper::<WampResult, D>(wamp_components)?)),
            Subscribe::ID => Ok(Self::Subscribe(helper::<Subscribe, D>(wamp_components)?)),
            Subscribed::ID => Ok(Self::Subscribed(helper::<Subscribed, D>(wamp_components)?)),
            Unregister::ID => Ok(Self::Unregister(helper::<Unregister, D>(wamp_components)?)),
            Unregistered::ID => Ok(Self::Unregistered(helper::<Unregistered, D>(wamp_components)?)),
            Unsubscribe::ID => Ok(Self::Unsubscribe(helper::<Unsubscribe, D>(wamp_components)?)),
            Unsubscribed::ID => Ok(Self::Unsubscribed(helper::<Unsubscribed, D>(wamp_components)?)),
            Welcome::ID => Ok(Self::Welcome(helper::<Welcome, D>(wamp_components)?)),
            Yield::ID => Ok(Self::Yield(helper::<Yield, D>(wamp_components)?)),
            _ => {
                Ok(Self::Extension(wamp_components))
            }
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::{hello::Hello, Messages};
    use serde_json::{json, to_string, from_str};

    #[test]
    fn hello_to_enum() {
        let h = Hello {
            realm: "some.random.realm".to_string(),
            details: json!({})
        };
        let s = to_string(&h).unwrap();
        let nh = from_str::<Messages>(&s).unwrap();
        if let Messages::Hello(nh) = nh {
            assert_eq!(nh, h)
        } else {
            panic!("Value is not a Hello message")
        }
    }
}