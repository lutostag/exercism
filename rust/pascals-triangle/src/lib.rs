pub struct PascalsTriangle {
    row_count: usize,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            row_count: row_count as usize,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows = Vec::with_capacity(self.row_count);
        let zero = &vec![0];
        while rows.len() < self.row_count {
            if rows.is_empty() {
                rows.push(vec![1]);
                continue;
            }
            let prev = &rows[rows.len() - 1];

            let prev_zero = prev.iter().chain(zero.iter());
            let zero_prev = zero.iter().chain(prev.iter());
            let row = prev_zero.zip(zero_prev).map(|(a, b)| a + b).collect();
            rows.push(row);
        }
        rows
    }
}
