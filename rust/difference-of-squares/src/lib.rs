pub mod for_loop {
    pub fn square_of_sum(n: usize) -> usize {
        let mut sum = 0;
        for i in 1..=n {
            sum += i;
        }
        sum * sum
    }

    pub fn sum_of_squares(n: usize) -> usize {
        let mut sum = 0;
        for i in 1..=n {
            sum += i * i;
        }
        sum
    }
}

pub mod map {
    pub fn sum_of_squares(n: usize) -> usize {
        (1..=n).map(|x| x.pow(2)).sum()
    }

    pub fn square_of_sum(n: usize) -> usize {
        (1..=n).sum::<usize>().pow(2)
    }
}

pub mod fold {
    pub fn sum_of_squares(n: usize) -> usize {
        (1..=n).fold(0, |acc, x| acc + x * x)
    }

    pub fn square_of_sum(n: usize) -> usize {
        (1..=n).fold(0, |acc, x| acc + x).pow(2)
    }
}

pub mod math {
    /// https://trans4mind.com/personal_development/mathematics/series/sumNaturalSquares.htm
    pub fn sum_of_squares(n: usize) -> usize {
        (n * (n + 1) * (2 * n + 1)) / 6
    }

    /// https://www.mathsisfun.com/algebra/sequences-sums-arithmetic.html
    pub fn square_of_sum(n: usize) -> usize {
        (n * (n + 1) / 2).pow(2)
    }
}

pub use map::*;

pub fn difference(n: usize) -> usize {
    return square_of_sum(n) - sum_of_squares(n);
}
