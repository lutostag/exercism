pub fn square_of_sum(n: usize) -> usize {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    return sum * sum;
}

pub fn sum_of_squares(n: usize) -> usize {
    let mut sum = 0;
    for i in 1..=n {
        sum += i * i;
    }
    return sum;
}

pub fn difference(n: usize) -> usize {
    return square_of_sum(n) - sum_of_squares(n);
}
