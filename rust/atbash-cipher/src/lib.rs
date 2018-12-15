#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";
lazy_static! {
    static ref ATBASH_KEY: HashMap<char, char> =
        ALPHABET.chars().rev().zip(ALPHABET.chars()).collect();
}

fn filter_swap<'a>(text: &'a str) -> impl Iterator<Item = char> + 'a {
    text.chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_lowercase())
        .map(|c| ATBASH_KEY.get(&c).unwrap_or(&c).clone())
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    filter_swap(cipher).collect()
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut spaced = String::with_capacity(plain.len());
    for (idx, c) in filter_swap(plain).enumerate() {
        spaced.push(c);
        if (idx + 1) % 5 == 0 {
            spaced.push(' ');
        }
    }
    String::from(spaced.trim())
}
