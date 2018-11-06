use std::collections::{BTreeMap, BTreeSet};

pub struct School {
    grades: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let students = self.grades.entry(grade).or_insert(BTreeSet::new());
        students.insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.grades.get(&grade).map(|s| s.iter().cloned().collect())
    }
}
