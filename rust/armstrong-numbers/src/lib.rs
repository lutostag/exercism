pub fn is_armstrong_number(num: u32) -> bool {
    let mut sum = 0_u32;
    let mut cur = num;

    let digits = num.to_string().len() as u32;
    while cur > 0 {
        // use a checked_add to not panic if we overflow
        match sum.checked_add((cur % 10).pow(digits)) {
            Some(safe) => sum = safe,
            None => return false,
        }
        cur /= 10;
    }
    sum == num
}
