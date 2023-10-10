use core::{client::{Client, WampRequest}, protocol::messages::{Messages, Authenticate}, hello, authenticate, subscribe, call};
use std::{thread::sleep, time::Duration, fmt::format};
use dotenv;
use serde_json::{json, to_string};


fn main() {
    dotenv::from_filename("examples/.env").unwrap();
    
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

    sleep(Duration::from_secs(3));
    

    loop {
        let m = client.read().unwrap();
        if let Some(message) = m {
            if let Messages::Challenge(cha) = message.clone() {
                client.send(authenticate!(dotenv::var("BEARER2").unwrap())).unwrap();
            }
            if let Messages::Welcome(welcome) = message.clone() {
                let auth_id = &welcome.details["authid"].as_str().unwrap();
                println!("auth id: {auth_id}");
                println!("{:#?}, {:#?}", subscribe!(format!("co.fun.chat.user.{auth_id}.invites", )), subscribe!(format!("co.fun.chat.user.{auth_id}.chats")));
                client.send(subscribe!(format!("co.fun.chat.user.{auth_id}.invites"))).unwrap();
                client.send(subscribe!(format!("co.fun.chat.user.{auth_id}.chats"))).unwrap();
                client.send(call!("co.fun.chat.get_or_create_chat".to_string(), json!({}), kwargs: json!({
                    "type": 1, 
                    "name": format!("{}_64e74dc756b2323ae00a9142", auth_id),
                    "users": [
                        "64e74dc756b2323ae00a9142"
                    ]
                }))).unwrap();
            };
            println!("{:#?}", message)
        }
        //let m:Messages = match m {
        //    Ok(m) => {
        //        Ok(m)
        //    },
        //    Err(e) => {
        //        println!("{:#?}", e);
        //        Err(e)
        //    }
        //}.unwrap();
        //println!("{:#?}", m);
        //if let Messages::Abort(a) = &m {
        //    println!("{:#?}", a);
        //}
        //if let Messages::Challenge(_c) = &m {
        //    client.send(authenticate!(dotenv::var("BEARER").unwrap())).unwrap();
        //}
    }

}

#[test]
fn poo() {
    let pee = "nsf".to_string();
    println!("{}", format!("some.event.{pee}.foo.bar"))
}