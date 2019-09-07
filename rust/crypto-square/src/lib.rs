pub fn encrypt(input: &str) -> String {
    let mut normalized: String = input
        .to_ascii_lowercase()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect();
    let sqrt = (normalized.len() as f64).sqrt();
    let (rows, cols) = (sqrt.floor() as usize, sqrt.ceil() as usize);
    let padding = " ".repeat((rows * cols) - normalized.len());
    normalized.push_str(&padding);

    let mut rect = vec![String::with_capacity(rows); cols];
    for (idx, c) in normalized.char_indices() {
        rect[idx % cols].push(c);
    }
    rect.join(" ")
}
