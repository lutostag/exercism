extern crate char_iter;

pub fn get_diamond(until: char) -> Vec<String> {
    let mut diamond = Vec::new();
    let back: String = char_iter::new('A', until).collect();
    let front: String = back.chars().rev().collect();
    let mut full = String::from(front);
    full.pop(); // remove second copy of 'A'
    full.push_str(&back);

    for c in char_iter::new('A', until) {
        let row: String = full.chars().map(|l| if l == c { c } else { ' ' }).collect();
        let middle_vec = diamond.len() / 2;
        if c != until {
            // Do not add 2-copies of the last character, only one
            diamond.insert(middle_vec, row.clone());
        }
        diamond.insert(middle_vec, row);
    }
    diamond
}
