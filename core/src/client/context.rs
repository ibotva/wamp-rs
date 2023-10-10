use std::{net::TcpStream, sync::{Arc, Mutex}};
use tungstenite::{WebSocket, stream::MaybeTlsStream, Message};
use crate::{protocol::messages::*, error::Error};

use super::client::{self, Socket};

pub(crate) type CallBack<T> = Box<dyn FnMut(Context, T) -> Context>;
pub(crate) type CallBackResult<T> = CallBack<Result<T, WampError>>;
pub(crate) type CallBackVec<K, V> = Vec<(K, CallBack<V>)>;
pub(crate) type CallBackVecResult<K, V> = CallBackVec<K, Result<V, WampError>>;


macro_rules! create_push_methods {
    ($method_name: ident, $vec_name: ident, $var_type: ident, $callback: ty) => {
        pub fn $method_name(&mut self, $method_name: $var_type, callback: $callback) -> Result<(), Error> {
            self.send($method_name.clone())?;
            Ok(self.$vec_name.push(($method_name, callback)))
        }
    };

    ($method_name: ident, $vec_name: ident, $var_type: ident, $callback: ty, no_send) => {
        pub fn $method_name(&mut self, $method_name: $var_type, callback: $callback) -> Result<(), Error> {
            Ok(self.$vec_name.push(($method_name, callback)))
        }
    };
}

macro_rules! create_find_methods {
    ($method_name: ident, $var_name: ident, $vec_name: ident, $return_type: ident, $var_type: ident) => {
        pub(crate) fn $method_name(&mut self, $var_name: &$var_type) -> Option<&mut ($return_type, CallBackResult<$var_type>)> {
            self.$vec_name.iter_mut().find(|i| i.0.request_id == $var_name.request_id)
        }
    };
    (event: $method_name: ident, $var_name: ident, $vec_name: ident, $return_type: ident, $var_type: ident) => {
        pub(crate) fn $method_name(&mut self, $var_name: &$var_type) -> Option<&mut ($return_type, CallBack<$var_type>)> {
            self.$vec_name.iter_mut().find(|i| i.0.subscription == $var_name.subscription)
        }
    };
}

macro_rules! create_find_by_error_method {
    ($method_name: ident, $method_type: ident, $vec_name: ident, $return_type: ident) => {
        pub fn $method_name(&mut self, error: &WampError) -> Option<&mut ($method_type, CallBackResult<$return_type>)> {
            if let Some(info) = self.$vec_name.iter_mut().find(|($method_name, _)| $method_name.request_id == error.request_id) {
                Some(info)
            } else {
                None
            }
        }
    };
}

pub struct Context {
    pub(crate) socket: Option<client::Socket>,
    pub(crate) registrations: CallBackVecResult<Register, Registered>,
    pub(crate) unregistrations: CallBackVecResult<Unregister, Unregistered>,
    pub(crate) subscriptions: CallBackVecResult<Subscribe, Subscribed>,
    pub(crate) unsubscriptions: CallBackVecResult<Unsubscribe, Unsubscribed>,
    pub(crate) publications: CallBackVecResult<Publish, Published>,
    pub(crate) calls: CallBackVecResult<Call, WampResult>,
    pub(crate) events: CallBackVec<Subscribed, Event>,
    pub(crate) invocations: CallBackVecResult<Registered, Invocation>,
    pub(crate) errors: CallBackVecResult<Messages, WampError>,
    pub(crate) messages: Vec<Message>,
    pub(crate) cancelations: CallBackVecResult<Cancel, Interrupt>
}

impl Context {
    pub fn new(socket: Option<Socket>) -> Self {
        Self {
            socket: socket,
            registrations: vec![],
            unregistrations: vec![],
            subscriptions: vec![],
            unsubscriptions: vec![],
            publications: vec![],
            calls: vec![],
            events: vec![],
            invocations: vec![],
            messages: vec![],
            errors: vec![],
            cancelations: vec![]
        }
    }

    pub fn new_with_capacity(socket: Option<Socket>, capacity: usize) -> Self {
        Self {
            socket: socket,
            registrations: Vec::with_capacity(capacity), 
            unregistrations: Vec::with_capacity(capacity), 
            subscriptions: Vec::with_capacity(capacity), 
            unsubscriptions: Vec::with_capacity(capacity), 
            publications: Vec::with_capacity(capacity), 
            calls: Vec::with_capacity(capacity), 
            events: Vec::with_capacity(capacity), 
            invocations: Vec::with_capacity(capacity),
            messages: Vec::with_capacity(capacity),
            errors: Vec::with_capacity(capacity),
            cancelations: Vec::with_capacity(capacity)
        }
    }

    pub fn send<T: TryInto<Message>>(&mut self, message: T) -> Result<(), Error>
    where 
        Error: From<<T as TryInto<Message>>::Error>
    {
        if let Some(socket) = &self.socket {
            let socket = &mut *socket.lock().unwrap();
           Ok(socket.send(message.try_into()?)?)
        } else {
            Ok(self.messages.push(message.try_into()?))
        }
    }

    create_find_by_error_method!(find_by_error_unsubscribe, Unsubscribe, unsubscriptions, Unsubscribed);
    create_find_by_error_method!(find_by_error_subscribe, Subscribe, subscriptions, Subscribed);
    create_find_by_error_method!(find_by_error_publish, Publish, publications, Published);
    create_find_by_error_method!(find_by_error_register, Register, registrations, Registered);
    create_find_by_error_method!(find_by_error_unregister, Unregister, unregistrations, Unregistered);
    create_find_by_error_method!(find_by_error_cancel, Cancel, cancelations, Interrupt);
    create_find_by_error_method!(find_by_error_call, Call, calls, WampResult);

    create_push_methods!(register, registrations, Register, CallBackResult<Registered>);
    create_push_methods!(unregister, unregistrations, Unregister, CallBackResult<Unregistered>);
    create_push_methods!(event, events, Subscribed, CallBack<Event>, no_send);
    create_push_methods!(unsubscribe, unsubscriptions, Unsubscribe, CallBackResult<Unsubscribed>);
    create_push_methods!(subscribe, subscriptions, Subscribe, CallBackResult<Subscribed>);
    create_push_methods!(publish, publications, Publish, CallBackResult<Published>);
    create_push_methods!(call, calls, Call, CallBackResult<WampResult>);
    create_push_methods!(invocation, invocations, Registered, CallBackResult<Invocation>);
    create_push_methods!(cancel, cancelations, Cancel, CallBackResult<Interrupt>);

    create_find_methods!(find_register, registered, registrations, Register, Registered);
    create_find_methods!(find_unregister, unregister, unregistrations, Unregister, Unregistered);
    create_find_methods!(event: find_event, event, events, Subscribed, Event);
    create_find_methods!(find_unsubscribe, unsubscribe, unsubscriptions, Unsubscribe, Unsubscribed);
    create_find_methods!(find_subscribe, subscribe, subscriptions, Subscribe, Subscribed);
    create_find_methods!(find_publish, publish, publications, Publish, Published);
    create_find_methods!(find_call, call, calls, Call, WampResult);
    create_find_methods!(find_invocation, invocation, invocations, Registered, Invocation);

    pub fn extend(&mut self, ctx: Context) {
        self.registrations.extend(ctx.registrations);
        self.unregistrations.extend(ctx.unregistrations);
        self.events.extend(ctx.events);
        self.unsubscriptions.extend(ctx.unsubscriptions);
        self.subscriptions.extend(ctx.subscriptions);
        self.publications.extend(ctx.publications);
        self.calls.extend(ctx.calls);
        self.invocations.extend(ctx.invocations)
    }
}
