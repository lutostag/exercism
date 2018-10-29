use std::collections::HashSet;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T> where T: std::hash::Hash + std::cmp::Ord {
    fn uniq_sides(&self) -> usize {
        self.sides.iter().collect::<HashSet<&T>>().len()
    }

    pub fn is_equilateral(&self) -> bool {
        self.uniq_sides() == 1
    }

    pub fn is_isosceles(&self) -> bool {
        self.uniq_sides() == 2
    }

    pub fn is_scalene(&self) -> bool {
        self.uniq_sides() == 3
    }
}

impl Triangle<u64> {
    pub fn build(sides: [u64; 3]) -> Option<Triangle<u64>> {
        let min: &u64 = sides.iter().min().unwrap();
        let max: &u64 = sides.iter().max().unwrap();
        let sum: u64 = sides.iter().sum();
        if min <= &0 || (2 * max) > sum {
            return None;
        }
        Some(Triangle { sides })
    }
}

// impl<'a, T> Triangle<&'a T> where T: std::cmp::Ord + std::iter::Sum<&'a T>, &'a T: std::cmp::PartialOrd + std::ops::Add<&'a T> {
//     pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
//         let max: &T = sides.iter().max().unwrap();
//         let sum: T = sides.iter().sum();
//         if sides.contains(&0) || (max + max) > sum {
//             return None;
//         }
//         Some(Triangle { sides })
//     }
// }

