#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;

use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref CLOSE_BRACKETS: HashMap<char, char> = hashmap! {
        '}' => '{',
        ')' => '(',
        ']' => '[',
    };
    static ref OPEN_BRACKETS: HashSet<char> =
        CLOSE_BRACKETS.values().into_iter().cloned().collect();
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        if OPEN_BRACKETS.contains(&c) {
            stack.push(c);
        } else if CLOSE_BRACKETS.contains_key(&c) {
            if stack.pop() != CLOSE_BRACKETS.get(&c).cloned() {
                return false;
            }
        }
    }
    stack.is_empty()
}
