fn primes_from<'a>(sieve: &'a Vec<bool>) -> impl Iterator<Item = usize> + 'a {
    sieve.iter().enumerate().filter_map(|(idx, &p)| match p {
        true => Some(idx),
        false => None,
    })
}

pub fn primes_up_to(upper_bound: usize) -> Vec<u64> {
    let mut sieve = vec![true; upper_bound + 1];
    sieve[0] = false;
    sieve[1] = false;
    let mut next = Some(2);
    while let Some(prime) = next {
        sieve.chunks_mut(prime).skip(2).for_each(|p| p[0] = false);
        next = primes_from(&sieve).skip_while(|&p| p <= prime).next();
    }
    (move || primes_from(&sieve).map(|p| p as u64).collect())()
}
