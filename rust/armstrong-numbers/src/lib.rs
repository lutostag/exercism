pub fn is_armstrong_number(num: u32) -> bool {
    let mut sum = 0u32;
    let mut current = num;

    let digits = (num as f64).log10().ceil() as u32;
    for _ in 1..=digits {
        sum += (current % 10).pow(digits);
        current /= 10;
    }
    return sum == num;
}
