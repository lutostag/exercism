/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

fn err_unless_coprime(a: i32) -> Result<(), AffineCipherError> {
    if a % 2 == 0 || a % 13 == 0 {
        Err(AffineCipherError::NotCoprime(a))
    } else {
        Ok(())
    }
}

fn transcode<'a, F>(text: &'a str, converter: F) -> impl Iterator<Item = char> + 'a
where
    F: Fn(i32) -> i32 + 'a,
{
    text.chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_lowercase())
        .map(move |c| match c {
            'a'..='z' => (converter(i32::from(c as u8 - b'a')) as u8 + b'a') as char,
            _ => c,
        })
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    err_unless_coprime(a)?;
    let encoded: Vec<_> = transcode(plaintext, |c| (a * c + b).rem_euclid(26)).collect();
    let split: Vec<String> = encoded.chunks(5).map(|c| c.iter().collect()).collect();
    Ok(split.join(" "))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    err_unless_coprime(a)?;
    let m_inv = (0..).find(|inv| (a * inv).rem_euclid(26) == 1).unwrap();
    Ok(transcode(ciphertext, |c| (m_inv * (c - b)).rem_euclid(26)).collect())
}
