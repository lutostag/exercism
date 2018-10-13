pub fn build_proverb(list: Vec<&str>) -> String {
    let mut proverb = String::new();
    
    for (a, b) in list.iter().zip(list.iter().skip(1)) {
        proverb += &format!("For want of a {} the {} was lost.\n", a, b);
    }
    if let Some(first) = list.first() {
        proverb += &format!("And all for the want of a {}.", first);
    }
    proverb
}
