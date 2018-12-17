extern crate char_iter;

fn offset(c: char) -> usize {
    if !(c.is_ascii_alphabetic() && c.is_ascii_uppercase()) {
        panic!("not an expected char: {}", c);
    }
    c as usize - 'A' as usize
}

pub fn get_diamond(until: char) -> Vec<String> {
    let mut diamond = Vec::new();
    let middle_str = offset(until);
    let blank = " ".repeat(middle_str * 2 + 1);

    for c in char_iter::new('A', until) {
        let mut row = blank.clone();
        let offset = offset(c);
        row.replace_range(middle_str + offset..middle_str + offset + 1, &c.to_string());
        row.replace_range(middle_str - offset..middle_str - offset + 1, &c.to_string());
        let middle_vec = diamond.len() / 2;
        if c != until {
            diamond.insert(middle_vec, row.clone());
        }
        diamond.insert(middle_vec, row);
    }
    diamond
}
