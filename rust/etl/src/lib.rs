use std::collections::BTreeMap;

pub fn transform(old_lookup: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut new_lookup = BTreeMap::new();
    for (&score, letters) in old_lookup {
        for &letter in letters {
            new_lookup.insert(letter.to_ascii_lowercase(), score);
        }
    }
    new_lookup
}
