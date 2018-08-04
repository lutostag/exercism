pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut sequences = Vec::new();

    if len > digits.len() {
        return sequences;
    }

    for i in 0..=digits.len() - len {
        let sequence = String::from(&digits[i..i + len]);
        sequences.push(sequence);
    }
    return sequences;
}
