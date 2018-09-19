/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.replace("-", "");
    let mut sum = 0;

    // use rev() to not deal with signedness/panics and avoid excessive typing
    for (idx, c) in isbn.chars().rev().enumerate() {
        let multiplier = idx as u32 + 1;
        match c {
            '0'...'9' => sum += c.to_digit(10).unwrap() * multiplier,
            'X' if idx == 0 => sum += 10,
            _ => return false,
        }
    }
    sum % 11 == 0 && isbn.len() == 10
}
