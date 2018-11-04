#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    } else if let Some(c) = string_digits.chars().find(|&c| !c.is_digit(10)) {
        return Err(Error::InvalidDigit(c));
    }

    let digits: Vec<u64> = string_digits
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();
    digits
        .windows(span)
        .map(|window| window.iter().product())
        .max()
        .ok_or(Error::SpanTooLong)
}
