pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;
    for i in 1..limit {
        for factor in factors {
            if i % factor == 0 {
                sum += i;
                break;
            }
        }
    }
    return sum;
}
