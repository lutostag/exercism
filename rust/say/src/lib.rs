pub fn encode(n: u64) -> String {
    match n {
        0 => format!("zero"),
        1 => format!("one"),
        2 => format!("two"),
        3 => format!("three"),
        4 => format!("four"),
        5 => format!("five"),
        6 => format!("six"),
        7 => format!("seven"),
        8 => format!("eight"),
        9 => format!("nine"),
        10 => format!("ten"),
        11 => format!("eleven"),
        12 => format!("twelve"),
        13 => format!("thirteen"),
        15 => format!("fifteen"),
        13...19 => format!("{}teen", encode(n % 10)),
        20 => format!("twenty"),
        30 => format!("thirty"),
        40 => format!("forty"),
        50 => format!("fifty"),
        60 => format!("sixty"),
        70 => format!("seventy"),
        80 => format!("eighty"),
        90 => format!("ninety"),
        21...99 => format!("{}-{}", encode(n - (n % 10)), encode(n % 10)),
        100...900 if n % 100 == 0 => format!("{} hundred", encode(n / 100)),
        100...999 => format!("{} {}", encode(n - (n % 100)), encode(n % 100)),
        _ => [
            "",
            " thousand",
            " million",
            " billion",
            " trillion",
            " quadrillion",
            " quintillion",
        ]
            .iter()
            .enumerate()
            .filter_map(|(idx, &sep)| match (n / 1000_u64.pow(idx as u32)) % 1000 {
                0 => None,
                part => Some(format!("{}{}", encode(part), &sep)),
            }).rev()
            .collect::<Vec<String>>()
            .join(" "),
    }
}
