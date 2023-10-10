use std::{sync::{Arc, Mutex}, net::TcpStream, f32::consts::E};
use http::Response;
use serde_json::from_str;
use tungstenite::{WebSocket, stream::MaybeTlsStream, connect, Message};
use core::{error::Error, protocol::messages::{Register, Registered, Messages, challenge::{self, Challenge}, hello, invocation::{self, Invocation}, WampError, WampErrorEvent, unsubscribe::{self, Unsubscribe}, publish::{self, Publish}, unregister::{self, Unregister}, subscribe::{self, Subscribe}, cancel::{self, Cancel}, Welcome, Published, Unregistered, Event, Subscribed, Unsubscribed, WampResult, Call, Interrupt}};

use super::{context::{Context, CallBackResult, CallBack, self}, WampRequest};

pub(crate) type Socket = Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>;

pub struct Client {
    pub socket: Socket,
    pub context: Context,
    on_welcome: Option<super::context::CallBack<Welcome>>,
    on_challenge: Option<super::context::CallBack<Challenge>>
}

macro_rules! client_context_link {
    ($method_name: ident, $method_type: ident, $callback: ty) => {
        pub fn $method_name(&mut self, $method_name: $method_type, callback: $callback) -> Result<(), Error> {
            self.context.$method_name($method_name, callback)
        }
    };
}

impl Client {
    pub fn connect<U: ToString, P: ToString>(request: WampRequest<U, P>) -> Result<(Client, Response<Option<Vec<u8>>>), Error> {
        let (socket, response) = connect(request)?;
        Ok((
            Self { 
                socket: Arc::new(Mutex::new(socket)), 
                context: Context::new(None),
                on_welcome: None,
                on_challenge: None
            },
            response
        ))
    }

    client_context_link!(publish, Publish, CallBackResult<Published>);
    client_context_link!(register, Register, CallBackResult<Registered>);
    client_context_link!(unregister, Unregister, CallBackResult<Unregistered>);
    client_context_link!(event, Subscribed, CallBack<Event>);
    client_context_link!(unsubscribe, Unsubscribe, CallBackResult<Unsubscribed>);
    client_context_link!(subscribe, Subscribe, CallBackResult<Subscribed>);
    client_context_link!(call, Call, CallBackResult<WampResult>);
    client_context_link!(invocation, Registered, CallBackResult<Invocation>);
    client_context_link!(cancel, Cancel, CallBackResult<Interrupt>);

    

    pub fn on_welcome(&mut self, on_welcome: CallBack<Welcome>) -> &mut Self {
        self.on_welcome = Some(on_welcome);
        self
    }

    pub fn on_challenge(&mut self, on_challenge: CallBack<Challenge>) -> &mut Self {
        self.on_challenge = Some(on_challenge);
        self
    }

    pub fn handle_and_empty_contexts(&mut self, message: Messages) -> Result<Option<Messages>, Error> {
            match message {
                Messages::Error(error) => {
                    match error.event {
                        WampErrorEvent::Call => {
                            Ok(None)
                        }
                        WampErrorEvent::Unsubscribe => {
                            self.context.unsubscriptions.retain(|(unsubscribe, _)| unsubscribe.request_id == error.request_id);
                            Ok(Some(Messages::from(error)))
                        },
                        WampErrorEvent::Subscribe => {
                            self.context.subscriptions.retain(|(subscribe, _)| subscribe.request_id == error.request_id);
                            Ok(Some(Messages::from(error)))
                        },
                        WampErrorEvent::Publish => {
                            self.context.publications.retain(|(publish, _)| publish.request_id == error.request_id);
                            Ok(Some(Messages::from(error)))
                        },
                        WampErrorEvent::Register => {
                            self.context.registrations.retain(|(register, _)| register.request_id == error.request_id);
                            Ok(Some(Messages::from(error)))
                        },
                        WampErrorEvent::Unregister => {
                            self.context.unregistrations.retain(|(unregister, _)| unregister.request_id == error.request_id);
                            Ok(Some(Messages::from(error)))
                        },
                        WampErrorEvent::Invocation => {
                            self.context.invocations.retain(|(invocation, _)| invocation.request_id == error.request_id);
                            Ok(Some(Messages::from(error)))
                        },
                        WampErrorEvent::Cancel => {
                            self.context.invocations.retain(|(invocation, _)| invocation.request_id == error.request_id);
                            Ok(Some(Messages::from(error)))
                        }
                    }
                }
                _ => {
                    Ok(None)
                }
            

        }
    }

    pub fn event_loop(&mut self) -> Result<(), Error> {
        loop {
            let message = self.read()?;
            if let Some(_) = message {
                self.read_contexts(message)?;
            } 
        }
        Ok(())
    }

    pub fn read_contexts(&mut self, message: Option<Messages>) -> Result<Option<Messages>, Error> {
        let ctx = self.get_message_context(message)?;
        let ctx = self.extend_context(ctx)?;
        Ok(ctx)
    }

    fn extend_context(&mut self, contexts: Option<(Messages, Option<Context>)>) -> Result<Option<Messages>, Error> {
        if let Some(message) = contexts {
            if let Some(context) = message.1 {
                self.context.extend(context);
            }
            return Ok(Some(message.0))
        } else {
            Ok(None)
        }
    }

    fn get_message_context(&mut self, message: Option<Messages>) -> Result<Option<(Messages, Option<Context>)>, Error> {
        match message {
            Some(message) => {
                match message {
                    Messages::Abort(abort) => Err(Error::Abort(abort)),
                    Messages::Error(error) => {
                        match error.event {
                            WampErrorEvent::Call => {
                                if let Some((call, callback)) = self.context.find_by_error_call(&error) {
                                    let context = callback(Context::new(Some(Arc::clone(&self.socket))), Err(error.clone()));
                                    Ok(Some((Messages::from(error), Some(context))))
                                } else {
                                    Ok(Some((Messages::from(error), None)))
                                }
                            },
                            WampErrorEvent::Unsubscribe => {
                                if let Some((unsubscribe, callback)) = self.context.find_by_error_unsubscribe(&error) {
                                    let context = callback(Context::new(Some(Arc::clone(&self.socket))), Err(error.clone()));
                                    Ok(Some((Messages::from(error), Some(context))))
                                } else {
                                    Ok(Some((Messages::from(error), None)))
                                }
                            },
                            WampErrorEvent::Subscribe => {
                                if let Some((subscribe, callback)) = self.context.find_by_error_subscribe(&error) {
                                    let context = callback(Context::new(Some(Arc::clone(&self.socket))), Err(error.clone()));
                                    Ok(Some((Messages::from(error), Some(context))))
                                } else {
                                    Ok(Some((Messages::from(error), None)))
                                }
                            },
                            WampErrorEvent::Publish => {
                                if let Some((publish, callback)) = self.context.find_by_error_publish(&error) {
                                    let context = callback(Context::new(Some(Arc::clone(&self.socket))), Err(error.clone()));
                                    Ok(Some((Messages::from(error), Some(context))))
                                } else {
                                    Ok(Some((Messages::from(error), None)))
                                }
                            },
                            WampErrorEvent::Register => {
                                if let Some((register, callback)) = self.context.find_by_error_register(&error) {
                                    let context = callback(Context::new(Some(Arc::clone(&self.socket))), Err(error.clone()));
                                    Ok(Some((Messages::from(error), Some(context))))
                                } else {
                                    Ok(Some((Messages::from(error), None)))
                                }
                            },
                            WampErrorEvent::Unregister => {
                                if let Some((unregister, callback)) = self.context.find_by_error_unregister(&error) {
                                    let context = callback(Context::new(Some(Arc::clone(&self.socket))), Err(error.clone()));
                                    Ok(Some((Messages::from(error), Some(context))))
                                } else {
                                    Ok(Some((Messages::from(error), None)))
                                }
                            },
                            WampErrorEvent::Invocation => {
                                Ok(Some((Messages::from(error), None)))
                            },
                            WampErrorEvent::Cancel => {
                                if let Some((cancel, callback)) = self.context.find_by_error_cancel(&error) {
                                    let context = callback(Context::new(Some(self.socket.clone())), Err(error.clone()));
                                    Ok(Some((Messages::from(error), Some(context))))
                                } else {
                                    Ok(Some((Messages::from(error), None)))
                                }
                            }
                        }
                    },
                    Messages::Event(event) => {
                        if let Some((subscribe, callback)) = self.context.events.iter_mut().find(|(subscribed, _)| subscribed.subscription == event.subscription) {
                            let context = callback(Context::new(Some(Arc::clone(&self.socket))), event.clone());
                            Ok(Some((Messages::from(event), Some(context))))
                        } else {
                            Ok(Some((Messages::from(event), None)))
                        }
                    },
                    Messages::Goodbye(goodbye) => {
                        todo!("Goodbye handlers not implemented.")
                    },
                    Messages::Interrupt(interrupt) => {
                        if let Some((cancel, callback)) = self.context.cancelations.iter_mut().find(|(cancel, _)| cancel.request_id == interrupt.request_id) {
                            let context = callback(Context::new(Some(self.socket.clone())), Ok(interrupt.clone()));
                            Ok(Some((Messages::from(interrupt), Some(context))))
                        } else {
                            Ok(Some((Messages::from(interrupt), None)))
                        }
                    },
                    Messages::Published(published) => {
                        if let Some((publish, callback)) = self.context.publications.iter_mut().find(|(publish, _)| publish.request_id == published.request_id) {
                            let context = callback(Context::new(Some(Arc::clone(&self.socket))), Ok(published.clone()));
                            Ok(Some((Messages::from(published), Some(context))))
                        } else {
                            Ok(Some((Messages::from(published), None)))
                        }
                    },
                    Messages::Registered(registered) => {
                        if let Some((register, callback)) = self.context.registrations.iter_mut().find(|(register, _)| register.request_id == registered.request_id) {
                            let context = callback(Context::new(Some(Arc::clone(&self.socket))), Ok(registered.clone()));
                            Ok(Some((Messages::from(registered), Some(context))))
                        } else {
                            Ok(Some((Messages::from(registered), None)))
                        }
                    },
                    Messages::Result(result) => {
                        if let Some((call, callback)) = self.context.calls.iter_mut().find(|(call, _)| call.request_id == result.request_id) {
                            let context = callback(Context::new(Some(self.socket.clone())), Ok(result.clone()));
                            Ok(Some((Messages::from(result), Some(context))))
                        } else {
                            Ok(Some((Messages::from(result), None)))
                        }
                    },
                    Messages::Subscribed(subscribed) => {
                        if let Some((subscribe, callback)) = self.context.subscriptions.iter_mut().find(|(subscribe, _)| subscribe.request_id == subscribed.request_id) {
                            let context = callback(Context::new(Some(self.socket.clone())), Ok(subscribed.clone()));
                            Ok(Some((Messages::from(subscribed), Some(context))))
                        } else {
                            Ok(Some((Messages::from(subscribed), None)))
                        }
                    },
                    Messages::Unregistered(unregistered) => {
                        if let Some((unregister, callback)) = self.context.find_unregister(&unregistered) {
                            let context = callback(Context::new(Some(self.socket.clone())), Ok(unregistered.clone()));
                            Ok(Some((Messages::from(unregistered), Some(context))))
                        } else {
                            Ok(Some((Messages::from(unregistered), None)))
                        }
                    },
                    Messages::Unsubscribed(unsubscribed) => {
                        if let Some((unsubscribe, callback)) = self.context.unsubscriptions.iter_mut().find(|(unsubscribe, _)| unsubscribe.request_id == unsubscribed.request_id) {
                            let context = callback(Context::new(Some(self.socket.clone())), Ok(unsubscribed.clone()));
                            Ok(Some((Messages::from(unsubscribed), Some(context))))
                        } else {
                            Ok(Some((Messages::from(unsubscribed), None)))
                        }
                    },
                    Messages::Welcome(welcome) => {
                        if let Some(callback) = &mut self.on_welcome {
                            let context = callback(Context::new(Some(self.socket.clone())), welcome.clone());
                            Ok(Some((Messages::from(welcome), Some(context))))
                        } else {
                            Ok(Some((Messages::from(welcome), None)))
                        }
                    },
                    Messages::Challenge(challenge) => {
                        println!("challenge received");
                        if let Some(callback) = &mut self.on_challenge {
                            let context = callback(Context::new(Some(self.socket.clone())), challenge.clone());
                            Ok(Some((Messages::from(challenge), Some(context))))
                        } else {
                            Ok(Some((Messages::from(challenge), None)))
                        }
                    },
                    Messages::Extension(_) => todo!(),
                    Messages::Cancel(cancel) => Err(Error::InvalidFrameReceived(cancel.into())),
                    Messages::Call(call) => Err(Error::InvalidFrameReceived(call.into())),
                    Messages::Yield(r#yield) => Err(Error::InvalidFrameReceived(r#yield.into())),
                    Messages::Authenticate(authenticate) => Err(Error::InvalidFrameReceived(authenticate.into())),
                    Messages::Hello(hello) => Err(Error::InvalidFrameReceived(hello.into())),
                    Messages::Invocation(invocation) => Err(Error::InvalidFrameReceived(invocation.into())),
                    Messages::Publish(publish) => Err(Error::InvalidFrameReceived(publish.into())),
                    Messages::Register(register) => Err(Error::InvalidFrameReceived(register.into())),
                    Messages::Subscribe(subscribe) => Err(Error::InvalidFrameReceived(subscribe.into())),
                    Messages::Unregister(unregister) => Err(Error::InvalidFrameReceived(unregister.into())),
                    Messages::Unsubscribe(unsubscribe) => Err(Error::InvalidFrameReceived(unsubscribe.into())),
                }
            }
            None => { Ok(None) }
        }
    }

    pub fn read(&mut self) -> Result<Option<Messages>, Error> {
        let message = self.socket.lock().unwrap().read().unwrap();
        match message {
            Message::Text(message) => Ok(Some(from_str(&message)?)),
            Message::Ping(_) => Ok(None),
            Message::Close(_) => Ok(None),
            Message::Binary(_) => Err(Error::Error("Error: Binary frame received\n\nCurrently I have not added support for serialization beyond string json format. Please create an issue if you are interested in contributing. I am planning on implementing support for the msg_pack format as well.")),
            Message::Pong(_) => Ok(None),
            Message::Frame(_) => Err(Error::Error("frame received from tungstenite, which their docs say isnt possible\nif this happened, run.")),
        }
    }


    pub fn send<T: TryInto<Message>>(&mut self, message: T) -> Result<(), Error> 
    where
        Error: From<<T as TryInto<Message>>::Error>

    {
        let socket = &mut *self.socket.lock().unwrap();
        Ok(socket.send(message.try_into()?)?)
    }

}


