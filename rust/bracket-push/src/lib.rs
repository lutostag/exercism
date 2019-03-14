#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;

use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref OPEN_BRACKETS: HashSet<char> = hashset! {'{', '(', '[',};
    static ref CLOSE_BRACKETS: HashMap<char, char> = hashmap! {
        '}' => '{',
        ')' => '(',
        ']' => '[',
    };
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        if OPEN_BRACKETS.contains(&c) {
            stack.push(c);
        } else if let Some(&m) = CLOSE_BRACKETS.get(&c) {
            if Some(m) != stack.pop() {
                return false;
            }
        }
    }
    stack.is_empty()
}
