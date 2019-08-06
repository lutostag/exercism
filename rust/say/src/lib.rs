use std::u64::MAX;

pub fn encode(n: u64) -> String {
    let larger = |base, string| format!("{} {} {}", encode(n / base), string, encode(n % base));

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
        14 => format!("fourteen"),
        15 => format!("fifteen"),
        16 => format!("sixteen"),
        17 => format!("seventeen"),
        18 => format!("eighteen"),
        19 => format!("nineteen"),
        20 => format!("twenty"),
        30 => format!("thirty"),
        40 => format!("forty"),
        50 => format!("fifty"),
        60 => format!("sixty"),
        70 => format!("seventy"),
        80 => format!("eighty"),
        90 => format!("ninety"),
        21...99 => format!("{}-{}", encode(n / 10 * 10), encode(n % 10)),
        1_000_000_000_000_000_000...MAX => larger(1_E18 as u64, "quintillion"),
        1_000_000_000_000_000...MAX => larger(1_E15 as u64, "quadrillion"),
        1_000_000_000_000...MAX => larger(1_E12 as u64, "trillion"),
        1_000_000_000...MAX => larger(1_E9 as u64, "billion"),
        1_000_000...MAX => larger(1_E6 as u64, "million"),
        1_000...MAX => larger(1_E3 as u64, "thousand"),
        100...MAX => larger(1_E2 as u64, "hundred"),
    }
    .replace(" zero", "")
}
