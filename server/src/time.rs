use std::time::{SystemTime, UNIX_EPOCH};

pub struct Time;

impl Time {

    pub fn get_timestamp() -> i32 {
        return SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32;
    }
}