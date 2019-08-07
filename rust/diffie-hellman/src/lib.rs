use rand::{thread_rng, Rng};

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2, p)
}

/// https://en.wikipedia.org/wiki/Modular_exponentiation#Pseudocode
pub use public_key as secret;
pub fn public_key(modulus: u64, base: u64, mut expo: u64) -> u64 {
    let mut result: u64 = 1;
    let mut base: u64 = base % modulus;

    while expo > 0 {
        if expo % 2 == 1 {
            result = result.checked_mul(base).unwrap() % modulus;
        }
        base = base.checked_mul(base).unwrap() % modulus;
        expo >>= 1;
    }
    result
}
