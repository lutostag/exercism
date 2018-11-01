fn get_camel_capitals(word: &str) -> String {
    let mut camels = String::new();
    let mut was_lowercase = false;
    for (idx, c) in word.char_indices() {
        if idx == 0 || was_lowercase && c.is_uppercase() {
            camels.push(c.to_ascii_uppercase());
        }
        was_lowercase = c.is_lowercase();
    }
    camels
}

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| !c.is_alphabetic())
        .map(|word| get_camel_capitals(word))
        .collect()
}
