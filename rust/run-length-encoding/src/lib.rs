pub fn encode(source: &str) -> String {
    let mut encoded = String::new();
    let mut chars = source.chars().peekable();
    let mut count = 1;

    while let Some(c) = chars.next() {
        if Some(&c) == chars.peek() {
            count += 1;
        } else {
            if count > 1 {
                encoded.push_str(&count.to_string());
            }
            encoded.push(c);
            count = 1;
        }
    }
    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut count = 0;

    for c in source.chars() {
        if let Some(d) = c.to_digit(10) {
            count = count * 10 + d;
        } else {
            if count == 0 {
                count = 1;
            }
            decoded.push_str(&c.to_string().repeat(count as usize));
            count = 0;
        }
    }
    decoded
}
