#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    if let Some(&n) = number.iter().find(|&&n| n >= from_base) {
        return Err(Error::InvalidDigit(n));
    }

    let mut total: u32 = number
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, &n)| n * from_base.pow(idx as u32))
        .sum();

    let mut digits = Vec::new();
    while total > 0 {
        digits.push(total % to_base);
        total /= to_base;
    }
    Ok(digits.into_iter().rev().collect())
}
