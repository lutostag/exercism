/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let (numeric, non): (String, String) = code.chars().partition(char::is_ascii_digit);
    let sum: u32 = numeric
        .chars()
        .rev()
        .enumerate()
        .map(|(idx, c)| {
            let multiplier = (idx as u32 % 2) + 1;
            let mut digit = c.to_digit(10).unwrap() * multiplier;
            if digit > 9 {
                digit -= 9;
            }
            digit
        }).sum();
    sum % 10 == 0 && numeric.len() > 1 && non.trim().is_empty()
}
