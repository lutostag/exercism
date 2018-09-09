extern crate rand;

use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2, p)
}

/// https://en.wikipedia.org/wiki/Modular_exponentiation#cite_ref-schneier96p244_1-0
pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    let mut expo = a;
    let mut base = g;
    let mut result: u64 = 1;

    if p == 1 {
        return 0;
    }
    base = base % p;
    while expo > 0 {
        if expo % 2 == 1 {
            result = (result * base) % p;
        }
        expo = expo >> 1;
        base = (base * base) % p;
    }
    result
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    public_key(p, b_pub, a)
}
