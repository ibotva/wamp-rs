use tungstenite::http::header::{ToStrError, InvalidHeaderValue};
use crate::protocol::messages::{Abort, Authenticate, Call, Cancel, Challenge, WampError, WampResult, Event, Goodbye, Hello, Interrupt, Invocation, Publish, Published, Register, Registered, Subscribe, Subscribed, Unregister, Unregistered, Unsubscribe, Unsubscribed, Welcome, Yield, Messages};

#[derive(Debug)]
pub enum Error {
    InvalidURI,
    ToStrError(ToStrError),
    InvalidHeaderValue(InvalidHeaderValue),
    TungsteniteError(tungstenite::Error),
    SerdeJsonError(serde_json::Error),
    InvalidMessageEnumMember,
    Error(&'static str),
    InvalidFrameReceived(Messages),
    Close,
    Abort(Abort),
    NoSuchWampErrorType(Messages),
    NoSuchMessage
}


impl Error {

}


macro_rules! message_to_from {
    ($typ: ident) => {

        impl TryFrom<$typ> for tungstenite::Message {
            type Error = serde_json::Error;

            fn try_from(value: $typ) -> Result<tungstenite::Message, Self::Error> {
                Ok(tungstenite::Message::Text(serde_json::to_string(&value)?))
            }
        }
    };
}

//message_to_from!(Abort);
message_to_from!(Abort);
message_to_from!(Authenticate);
message_to_from!(Call);
message_to_from!(Cancel);
message_to_from!(Challenge);
message_to_from!(WampError);
message_to_from!(WampResult);
message_to_from!(Event);
message_to_from!(Goodbye);
message_to_from!(Hello);
message_to_from!(Interrupt);
message_to_from!(Invocation);
message_to_from!(Publish);
message_to_from!(Published);
message_to_from!(Register);
message_to_from!(Registered);
message_to_from!(Subscribe);
message_to_from!(Subscribed);
message_to_from!(Unregister);
message_to_from!(Unregistered);
message_to_from!(Unsubscribe );
message_to_from!(Unsubscribed);
message_to_from!(Welcome);
message_to_from!(Yield);




//impl<M: WampMessage + Serialize> TryFrom<M> for crate::error::Error {
//    type Error = Error;
//
//    fn try_from(value: M) -> Result<Self, Self::Error> {
//        
//    }
//}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJsonError(value)
    }
}

impl From<tungstenite::Error> for Error {
    fn from(value: tungstenite::Error) -> Self {
        Self::TungsteniteError(value)
    }
}

#[derive(Debug)]
pub enum WampErrorUri {
    NotAuthorized,
    ProcedureAlreadyExists,
    NoSuchRealm,
    ProtocolViolation,
    NoSuchSubscription,
    NoSuchRegistration,
    InvalidUri,
    NoSuchProcedure,
    InvalidArgument,
    Canceled,
    PayloadSizeExceeded,
    FeatureNotSupported,
    Timeout,
    Unavailable,
    NoAvailableCallee,
    DiscloseMeNotAllowed,
    OptionDisallowedDiscloseMe,
    NoMatchingAuthMethod,
    NoSuchRole,
    NoSuchPrincipal,
    AuthenticationDenied,
    AuthenticationFailed,
    AuthenticationRequired,
    AuthorizationDenied,
    AuthorizationFailed,
    AuthorizationRequired,
    NetworkFailure,
    OptionNotAllowed
}

pub enum CloseUri {
    SystemShutdown,
    CloseRealm,
    GoodbyeAndOut,
    Killed
}
