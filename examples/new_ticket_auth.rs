use core::{client::{Client, WampRequest}, protocol::messages::{Messages, Authenticate}, hello, authenticate, subscribe, call};
use std::{thread::sleep, time::{Duration, SystemTime}, fmt::format};
use dotenv;
use serde_json::{json, to_string};


fn main() {
    dotenv::from_filename("examples/.env").unwrap();
    let time = SystemTime::now();
    let (mut client, _) = Client::connect(WampRequest { uri: dotenv::var("URL").unwrap(), protocol: "wamp.json" }).unwrap();

    client.send(hello!{
        "co.fun.chat.ifunny".to_string(),
        json!({
            "roles": {
                "subscriber": {},
                "caller": {},
                "callee": {},
                "publisher": {}
            },
            "authmethods": ["ticket"]
        })
    }).unwrap();

    client.on_challenge(Box::new(|mut ctx, challenge| {
        ctx.send(authenticate!(dotenv::var("BEARER").unwrap())).unwrap();
        ctx
    }));

    client.on_welcome(Box::new(move |mut ctx, welcome| {
        let authid = &welcome.details["authid"].as_str().unwrap();
        let dur = SystemTime::now().duration_since(time);
        println!("{:#?}", dur.unwrap());
        println!("authid: {authid}");
        ctx.send(subscribe!(format!("co.fun.chat.user.{authid}.invites"))).unwrap();
        ctx.send(subscribe!(format!("co.fun.chat.user.{authid}.chats"))).unwrap();
        ctx
    }));

    let _ = client.event_loop().unwrap();
}

#[test]
fn poo() {
    let pee = "nsf".to_string();
    println!("{}", format!("some.event.{pee}.foo.bar"))
}