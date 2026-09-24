use std::thread;
use std::time::Duration;

pub fn wait(seconds: u64) {
    thread::sleep(Duration::from_secs(seconds));
}