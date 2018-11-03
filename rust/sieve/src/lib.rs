pub fn primes_up_to(upper_bound: usize) -> Vec<u64> {
    let mut numbers = vec![true; upper_bound + 1];
    let mut primes = Vec::new();
    let mut prime = 2;
    while prime <= upper_bound {
        primes.push(prime as u64);
        numbers[prime..].iter_mut().step_by(prime).for_each(
            |b| *b = false,
        );

        let next = numbers[prime..].iter().position(|&b| b).map(|p| p + prime);
        match next {
            Some(n) => prime = n,
            None => break,
        };
    }
    primes
}
