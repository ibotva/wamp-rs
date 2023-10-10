pub mod context;
//mod headers;
//pub use headers::{WebsocketHeaders, create_sec_protocol_header};
mod request;
pub use request::WampRequest;
pub use tungstenite::client::IntoClientRequest;
pub mod client;
pub use client::Client;



#[cfg(test)]
mod tests {

    use crate::client::WampRequest;

    #[test]
    fn test() {
        use tungstenite::connect;

        let req = WampRequest {
            uri: "wss://chat.ifunny.co/chat",
            protocol: "wamp.json"
        };

        let (mut s, _) = connect(req).unwrap();

        loop {
            //println!("{}", socket.read().unwrap())
            println!("{}", s.read().unwrap());
            //println!("{:#?}\n{}", e,e.to_string());
            //sleep(Duration::from_secs(3))
            break;
        }
    }
}