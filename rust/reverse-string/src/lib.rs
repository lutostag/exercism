extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut reversed = String::with_capacity(input.len());
    input
        .graphemes(true)
        .rev()
        .for_each(|cluster| reversed.push_str(cluster));
    reversed
}
