
use dotenv;
use serde_json::{json, Value};
use core::client::{Client, WampRequest};
use core::protocol::factories::{subscribe, subscription_contains, self};
use std::{time::SystemTime, sync::{Mutex, Arc}};


fn main() {
    dotenv::from_filename("examples/.env").unwrap();
    let time = SystemTime::now();
    let (mut client, _) = Client::connect(WampRequest { uri: dotenv::var("URL").unwrap(), protocol: "wamp.json" }).unwrap();

    client.send(core::hello!{
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

    client.on_challenge(Box::new(|mut ctx, _| {
        ctx.send(core::authenticate!(dotenv::var("BEARER").unwrap())).unwrap();
        ctx
    }));

    client.on_welcome(Box::new(move |mut ctx, welcome| {
        let authid = &welcome.details["authid"].as_str().unwrap();
        let dur = SystemTime::now().duration_since(time);
        println!("{:#?} {:#?}", dur.unwrap(), welcome);
        let _ = ctx.subscribe(core::subscribe!(format!("co.fun.chat.user.{authid}.chats")), Box::new(move |mut ctx, subscribed| {
            let subscribed = subscribed.unwrap();

            ctx.event(subscribed, Box::new(move |mut ctx, event| {
                let chats: &Vec<&Value> = &event.kwargs.get("chats").unwrap().as_array().unwrap().iter().map(|i| {
    
                    let topic = format!("co.fun.chat.chat.{}", i.get("name").unwrap().as_str().unwrap());
                    if !factories::subscription_contains(&topic) {
                        factories::subscribe(topic.clone());
                        ctx.subscribe(core::subscribe!(topic), Box::new(|mut ctx, subscribed| {
                        
                            println!("Subscription success: {:#?}", subscribed);
                            ctx.event(subscribed.unwrap(), Box::new(|ctx, event| {
                                println!("Individual chat fram: {:#?}", event);
                                ctx
                            })).unwrap();
                            ctx
                        })).unwrap();
                    }

                    
                    i
                }).collect();
                ctx
            })).unwrap();
            ctx
        })).unwrap();

        let _ = ctx.subscribe(core::subscribe!(format!("co.fun.chat.user.{authid}.invites")), Box::new(|mut ctx, subscribed| {
            let subscribed = subscribed.unwrap();
            ctx.event(subscribed, Box::new(|mut ctx, event| {
                println!("{:#?}", event);
                let chats = event.kwargs.get("chats");
                if let Some(chats) = chats {
                    let chats =chats.as_array().unwrap();
                    let ids = chats.iter().map(|i|{
                        i["name"].as_str().unwrap()
                    }).collect::<Vec<&str>>();
                    if !ids.is_empty() {
                        ctx.call(core::call!("co.fun.chat.invite.accept", json!({}), args: json!(ids)), Box::new(|ctx, result| {
                            let result = result.unwrap();
                            println!("{:#?}", result);
                            ctx
                        })).unwrap();
                    }
                }
                ctx
            })).unwrap();
            ctx
        })).unwrap();
        
        println!("authid: {authid}");
        ctx
    }));

    let _ = client.event_loop().unwrap();
}

#[test]
fn poo() {
    let pee = "nsf".to_string();
    println!("{}", format!("some.event.{pee}.foo.bar"))
}