use std::sync::{Arc, Mutex};
struct Increment(u128);

impl Increment {

    fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Increment(0)))
    }

    fn get_request_id(&mut self) -> u128 {
        self.0 = self.0 + 1;
        self.0
    }

}