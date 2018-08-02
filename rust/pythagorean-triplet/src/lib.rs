pub fn find() -> Option<u32> {
    let sum = 1000;
    let max_a = sum / 3; // assuming a <= b <= c so limit iterations accordingly

    for a in 1..=max_a {
        for b in 1..sum - a {
            let c = sum - a - b;
            if a * a + b * b == c * c {
                return Some(a * b * c);
            }
        }
    }
    return None;
}
