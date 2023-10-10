use std::sync::RwLock;

use lazy_static::lazy_static;

lazy_static! {
    static ref NUMBER: RwLock<u64> = RwLock::new(0);
    static ref TOPICS: RwLock<Vec<String>> = RwLock::new(vec![]);
}

pub fn increment() -> u64 {
    let previous = *NUMBER.read().unwrap();
    let mut num = NUMBER.write().unwrap();
    *num = previous + 1;
    *num
}

pub fn subscribe<T: ToString>(topic: T) {
    let mut current = TOPICS.write().unwrap();
    current.push(topic.to_string())
}

pub fn unsubscribe<T: ToString>(topic: &T) {
    let mut current = TOPICS.write().unwrap();
    current.retain(|i| i != &topic.to_string())
}

pub fn subscription_contains<T: ToString>(topic: &T) -> bool {
    TOPICS.read().unwrap().contains(&topic.to_string())
}