pub fn series(digits: &str, len: usize) -> Vec<String> {
    match digits.len().checked_sub(len) {
        Some(length) => (0..=length)
            .map(|idx| String::from(&digits[idx..idx + len]))
            .collect(),
        None => Vec::new(),
    }
}
