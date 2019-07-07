pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: PartialOrd,
{
    fn uniq_sides(&self) -> usize {
        let mut vec: Vec<_> = self.sides.iter().collect();
        vec.dedup();
        vec.len()
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
    T: Copy + Default + PartialOrd + std::ops::Add<T, Output = T>,
{
    pub fn build(mut sides: [T; 3]) -> Option<Triangle<T>> {
        if sides.iter().any(|s| T::default().partial_cmp(s).is_none()) {
            return None;
        }
        sides.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let (min, max) = (sides[0], sides[2]);
        let sum = sides.iter().fold(T::default(), |s, &i| s + i);
        if min <= T::default() || (max + max) > sum || (max + max) == max {
            return None;
        }
        Some(Triangle { sides })
    }
}
