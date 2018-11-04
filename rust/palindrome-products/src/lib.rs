extern crate rayon;
use rayon::prelude::*;

pub type Palindrome = u64;

fn is_palindrome(number: u64) -> bool {
    let mut num = number;
    let mut reversed = 0;
    while num > 0 {
        reversed = reversed * 10 + num % 10;
        num /= 10
    }
    reversed == number
}

pub fn get_palindrome_products(min: u64, max: u64) -> Vec<u64> {
    let palindromes = Vec::new();

    if min >= max {
        return palindromes;
    }

    (min..max + 1)
        .into_par_iter()
        .flat_map(|x| {
            (x..=max)
                .map(|y| x * y)
                .filter(|&p| is_palindrome(p))
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn min(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.first().cloned()
}

pub fn max(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().max().cloned()
}
