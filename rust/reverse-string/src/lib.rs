extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut reversed = String::with_capacity(input.len());
    for cluster in input.graphemes(true).rev() {
        reversed.push_str(cluster);
    }
    return reversed;
}
