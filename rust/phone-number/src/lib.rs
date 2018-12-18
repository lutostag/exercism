pub fn number(user_number: &str) -> Option<String> {
    let mut digits: String = user_number.chars().filter(char::is_ascii_digit).collect();
    if digits.len() == 11 && digits.starts_with('1') {
        digits.remove(0);
    }

    let digit_bytes = digits.as_bytes();
    if digits.len() == 10 && digit_bytes[0] > b'1' && digit_bytes[3] > b'1' {
        Some(digits)
    } else {
        None
    }
}
