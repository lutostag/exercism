#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref WORD: Regex = Regex::new(r"[\w\d]+('[\w\d]+)?").unwrap();
}

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut hash = HashMap::new();
    for word in WORD.find_iter(&words.to_lowercase()) {
        let counter = hash.entry(String::from(word.as_str())).or_insert(0);
        *counter += 1;
    }
    hash
}
