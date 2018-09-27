pub fn nth(n: u32) -> Option<u32> {
    let mut primes = Vec::with_capacity(n as usize);
    let mut current = 2_u32; // avoid 0 and 1 which are not prime

    while primes.len() < n as usize {
        if primes.iter().all(|&prime| current % prime != 0) {
            primes.push(current);
        }
        current += 1;
    }
    primes.pop()
}
