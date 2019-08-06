pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }
    let pivot = array.len() / 2;

    match array[pivot] {
        value if key == value => Some(pivot),
        value if key < value => find(&array[..pivot], key),
        _ => find(&array[pivot + 1..], key).map(|o| o + pivot + 1),
    }
}
