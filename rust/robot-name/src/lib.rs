#[macro_use]
extern crate lazy_static;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::sync::Mutex;

lazy_static! {
    static ref ROBOTS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

#[derive(Hash, PartialEq, Eq)]
pub struct Robot {
    name: String,
}

fn create_name() -> String {
    let letter = || thread_rng().gen_range(b'A', b'Z' + 1) as char;
    let digit = || thread_rng().gen_range(b'0', b'9' + 1) as char;
    let mut name;
    loop {
        name = format!("{}{}{}{}{}", letter(), letter(), digit(), digit(), digit());
        let mut robots = ROBOTS.lock().unwrap();
        if robots.len() >= 26 * 26 * 10 * 10 * 10 {
            drop(robots); // prevents poisoning of mutex but can still panic in this thread
            panic!("No unique robot names left")
        } else if robots.insert(name.to_string()) {
            break;
        }
    }
    name
}

fn delete_name(name: &str) -> () {
    let mut robots = ROBOTS.lock().unwrap();
    robots.remove(name);
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: create_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        delete_name(self.name());
        self.name = create_name();
    }
}

impl Drop for Robot {
    fn drop(&mut self) {
        delete_name(self.name());
    }
}
