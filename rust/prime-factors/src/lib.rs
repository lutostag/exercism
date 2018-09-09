pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = Vec::new();
    let mut n = n;
    let mut factor = 2_u64;
    
    while n > 1 {
        while n % factor == 0 {
            n /= factor;
            prime_factors.push(factor);
        }
        factor += 1;
    }
    prime_factors
}
