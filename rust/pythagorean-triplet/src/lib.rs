pub fn find() -> Option<u32> {
    let sum = 1000;

    for a in 1..sum {
        for b in 1..sum - a {
            let c = sum - a - b;
            if (c + b + a == sum) && (a * a + b * b == c * c) {
                return Some(a * b * c);
            }
        }
    }
    return None;
}
