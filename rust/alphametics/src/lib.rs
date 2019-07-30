use itertools::Itertools;
use permutohedron::Heap as Permutations;
use std::collections::{BTreeSet, HashMap};

fn check(input: &str, mapping: &HashMap<&char, &u8>) -> bool {
    let mut sum = 0;
    let mut num = 0;
    for c in input.chars() {
        if let '+' | '=' = c {
            sum += num;
            num = 0;
        } else if let 'A'..='Z' = c {
            let digit = **mapping.get(&c).unwrap() as u64;
            num = 10 * num + digit;
            if num == 0 {
                return false; // started a number with 0
            }
        }
    }
    sum == num
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let uniq_chars: BTreeSet<_> = input.chars().filter(char::is_ascii_uppercase).collect();

    for mut comb in (0..=9u8).combinations(uniq_chars.len()) {
        let mut perms = Permutations::new(&mut comb);
        while let Some(perm) = perms.next_permutation() {
            let mapping = uniq_chars.iter().zip(perm.iter()).collect();

            if check(input, &mapping) {
                return Some(mapping.iter().map(|(&&c, &&v)| (c, v)).collect());
            }
        }
    }
    None
}
