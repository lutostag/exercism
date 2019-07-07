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

impl<T> Triangle<T>
where
    T: Copy + Default + std::cmp::Ord + std::ops::Add<T, Output = T>,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let min = *sides.iter().min()?;
        let max = *sides.iter().max()?;
        let sum = sides.iter().fold(T::default(), |s, &i| s + i);
        if min <= T::default() || (max + max) > sum {
            return None;
        }
        Some(Triangle { sides })
    }
}
