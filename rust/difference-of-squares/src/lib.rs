pub fn square_of_sum_for(n: usize) -> usize {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum * sum
}

pub fn sum_of_squares_for(n: usize) -> usize {
    let mut sum = 0;
    for i in 1..=n {
        sum += i * i;
    }
    sum
}

pub fn sum_of_squares_fold(n: usize) -> usize {
    (1..=n).fold(0, |acc, x| acc + x * x)
}

pub fn sum_of_squares(n: usize) -> usize {
    (1..=n).map(|x| x.pow(2)).sum()
}

pub fn square_of_sum(n: usize) -> usize {
    (1..=n).sum::<usize>().pow(2)
}

pub fn difference(n: usize) -> usize {
    return square_of_sum(n) - sum_of_squares(n);
}
