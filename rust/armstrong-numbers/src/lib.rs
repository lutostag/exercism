pub fn is_armstrong_number(num: u32) -> bool {
    let mut sum = Some(0u32);
    let mut current = num;

    let digits = num.to_string().len() as u32;
    for _ in 1..=digits {
        let add = (current % 10).pow(digits);
        current /= 10;

        // use a checked_add to not panic if we overflow
        sum = sum.unwrap().checked_add(add);
        if sum.is_none() {
            return false;
        }
    }
    return sum.unwrap() == num;
}
