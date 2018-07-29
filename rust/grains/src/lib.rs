pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    return 1_u64 << (s - 1); // use bitshift, same as 2 raised to (s - 1)
}

pub fn total() -> u64 {
    let mut sum = 0;
    for s in 1..=64 {
        sum += square(s);
    }
    return sum;
}
