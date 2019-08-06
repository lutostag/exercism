#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

const MASK: u32 = 0b01111111;
const HIGH: u32 = 0b10000000;

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for mut v in values.iter().cloned().rev() {
        bytes.push((v & MASK) as u8);
        v = v >> 7;
        while v != 0 {
            bytes.push((v & MASK | HIGH) as u8);
            v = v >> 7;
        }
    }
    bytes.into_iter().rev().collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    if let Some(last) = bytes.last() {
        if last & (HIGH as u8) != 0 {
            return Err(Error::IncompleteNumber);
        }
    }

    let mut numbers = Vec::new();
    let mut number: u32 = 0;
    for &byte in bytes.iter() {
        if number.leading_zeros() < 7 {
            return Err(Error::Overflow);
        }
        number = number << 7 | (byte as u32 & MASK);
        if (byte & (HIGH as u8)) == 0 {
            numbers.push(number);
            number = 0;
        }
    }
    Ok(numbers)
}
