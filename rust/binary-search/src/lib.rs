pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let pivot = array.len() / 2;

    match array.len() {
        1 if key == array[0] => Some(0),
        0 | 1 => None,
        _ if key == array[pivot] => Some(pivot),
        _ if key < array[pivot] => find(&array[..pivot], key),
        _ => find(&array[pivot..], key).map(|o| o + pivot),
    }
}
