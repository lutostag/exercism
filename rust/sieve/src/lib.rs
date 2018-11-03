pub fn primes_up_to(upper_bound: usize) -> Vec<u64> {
    let mut sieve = vec![true; upper_bound + 1];
    let mut primes = Vec::new();
    let mut next = Some(2);
    while let Some(prime) = next {
        if prime <= upper_bound {
            primes.push(prime as u64);
        }
        sieve.iter_mut().step_by(prime).skip(1).for_each(
            |p| *p = false,
        );

        next = sieve.iter().skip(prime + 1).position(|&p| p).map(|idx| {
            idx + prime + 1
        });
    }
    primes
}
