#![feature(euclidean_division)]
use std::collections::HashMap;

const ALPHABET: &'static str = "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ";

pub fn rotate(input: &str, key: i8) -> String {
    let key: usize = key.rem_euclid(26) as usize * 2; // multiply for upper and lower
    let rotated = ALPHABET.chars().skip(key).chain(ALPHABET.chars().take(key));
    let cipher: HashMap<char, char> = ALPHABET.chars().zip(rotated).collect();

    input
        .clone()
        .chars()
        .map(|c| cipher.get(&c).unwrap_or(&c).clone())
        .collect()
}
