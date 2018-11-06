use std::collections::HashMap;

pub struct School {
    data: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let students = self.data.entry(grade).or_insert(Vec::new());
        students.push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.data.keys().cloned().collect();
        grades.sort();
        grades
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.data.get(&grade).map(|v| {
            let mut v = v.clone();
            v.sort();
            v
        })
    }
}
