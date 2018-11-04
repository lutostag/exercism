#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }

    let digits: Vec<u64> = string_digits
        .chars()
        .map(|c| c.to_digit(10)
             .ok_or(Error::InvalidDigit(c))
             .map(|d| d as u64))
        .collect::<Result<_, _>>()?;

    digits
        .windows(span)
        .map(|window| window.iter().product())
        .max()
        .ok_or(Error::SpanTooLong)
}
