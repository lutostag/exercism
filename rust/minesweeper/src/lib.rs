fn count_surrounding(y: usize, x: usize, field: &[&str]) -> char {
    let mut count = 0;
    for y in y.saturating_sub(1)..=(y + 1) {
        for x in x.saturating_sub(1)..=(x + 1) {
            if field.get(y).and_then(|r| r.get(x..=x)) == Some("*") {
                count += 1;
            }
        }
    }
    if count == 0 {
        return ' ';
    }
    count.to_string().pop().unwrap()
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut annotated = vec![String::new(); minefield.len()];
    for (y, row) in minefield.iter().enumerate() {
        for (x, c) in row.char_indices() {
            annotated[y].push(match c {
                '*' => '*',
                _ => count_surrounding(y, x, minefield),
            });
        }
    }
    annotated
}
