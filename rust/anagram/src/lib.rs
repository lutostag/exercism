use std::collections::{HashMap, HashSet};

fn count_letters(word: &str) -> HashMap<char, u16> {
    let mut letters = HashMap::new();
    for ch in word.chars() {
        let counter = letters.entry(ch).or_insert(0);
        *counter += 1;
    }
    letters
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let letters = count_letters(&word);

    possible_anagrams
        .iter()
        .filter(|w| {
            let w = &w.to_lowercase();
            letters == count_letters(w) && w != &word
        })
        .cloned()
        .collect()
}
