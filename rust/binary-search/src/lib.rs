pub fn find(array: &[i32], key: i32) -> Option<usize> {
    fn find_recursive(array: &[i32], key: i32, offset: usize) -> Option<usize> {
        let pivot = array.len() / 2;

        match array.len() {
            0 => None,
            1 if key == array[0] => Some(offset),
            1 => None,
            _ if key == array[pivot] => Some(offset + pivot),
            _ if key < array[pivot] => find_recursive(&array[..pivot], key, offset),
            _ => find_recursive(&array[pivot..], key, offset + pivot),
        }
    }
    find_recursive(array, key, 0)
}
