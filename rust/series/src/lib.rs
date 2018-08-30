pub fn series(digits: &str, len: usize) -> Vec<String> {
    match digits.len().checked_sub(len) {
        None => Vec::new(),
        Some(items) => (0..=items)
            .map(|i| String::from(&digits[i..i + len]))
            .collect(),
    }
}
