use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct CustomSet<T> {
    items: Vec<T>,
}

impl<T: Clone + Debug + PartialEq> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut set = CustomSet { items: Vec::new() };
        for i in input {
            set.add(i.clone());
        }
        set
    }

    pub fn contains(&self, element: &T) -> bool {
        self.items.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.items.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.items.iter().all(|i| other.contains(i))
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.items.iter().any(|i| other.contains(i))
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let mut set = Self::new(&[]);
        for i in self.items.iter().filter(|i| other.contains(i)) {
            set.add(i.clone());
        }
        set
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut set = Self::new(&[]);
        for i in self.items.iter().filter(|i| !other.contains(i)) {
            set.add(i.clone());
        }
        set
    }

    pub fn union(&self, other: &Self) -> Self {
        CustomSet::new(&[self.items.as_slice(), other.items.as_slice()].concat())
    }
}

impl<T: PartialEq> PartialEq for CustomSet<T> {
    fn eq(&self, other: &CustomSet<T>) -> bool {
        self.items.len() == other.items.len() && self.items.iter().all(|i| other.items.contains(i))
    }
}
