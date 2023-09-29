#[derive(Debug)]
pub enum Error {
    InvalidURI
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
