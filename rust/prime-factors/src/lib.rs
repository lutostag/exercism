pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = Vec::new();
    let mut factored = n;
    let mut possible = 2_u64;
    
    while factored > 1 {
        while factored % possible == 0 {
            factored /= possible;
            prime_factors.push(possible);
        }
        possible += 1;
    }
    return prime_factors;
}
