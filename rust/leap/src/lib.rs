fn divisible(number: i32, divisor: i32) -> bool {
    number % divisor == 0
}

pub fn is_leap_year(year: i32) -> bool {
    divisible(year, 4) && !divisible(year, 100) || divisible(year, 400)
}
