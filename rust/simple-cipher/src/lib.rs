use rand::distributions::{Alphanumeric, Distribution};
use rand::thread_rng;
use std::ops;

fn is_valid(string: &str) -> bool {
    string.chars().all(|c| c.is_ascii_lowercase()) && !string.is_empty()
}

fn transcode(key: &str, s: &str, op: fn(u8, u8) -> u8) -> Option<String> {
    if is_valid(key) && is_valid(s) {
        Some(
            s.bytes()
                .zip(key.bytes().cycle())
                .map(|(c, key)| (op(c - b'a' + 26, key - b'a') % 26 + b'a') as char)
                .collect(),
        )
    } else {
        None
    }
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    transcode(key, s, ops::Add::add)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    transcode(key, s, ops::Sub::sub)
}

pub fn encode_random(s: &str) -> (String, String) {
    let key: String = Alphanumeric
        .sample_iter(thread_rng())
        .filter(char::is_ascii_lowercase)
        .take(128)
        .collect();
    let encoded = encode(&key, s).expect("invalid cleartext");
    (key, encoded)
}
