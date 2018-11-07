use std::collections::HashSet;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: std::hash::Hash + std::cmp::Ord,
{
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

impl<'a, T: 'a> Triangle<T>
where
    T: std::cmp::Ord
        + std::iter::Sum<T>
        + std::ops::Add<T, Output = T>
        + std::marker::Copy
        + std::convert::From<u8>,
    &'a T: std::cmp::PartialOrd,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let max = sides.iter().cloned().max().unwrap();
        let min = sides.iter().cloned().min().unwrap();
        let twice_max = max + max;
        let sum = sides.iter().cloned().sum();
        let zero = T::from(0u8);
        if min <= zero || twice_max > sum {
            return None;
        }
        Some(Triangle { sides })
    }
}
