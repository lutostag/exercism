use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut uniq: HashSet<char> = HashSet::new();
    candidate
        .to_lowercase()
        .chars()
        .filter(|&c| c.is_alphanumeric())
        .all(|c| uniq.insert(c))
}
