pub fn build_proverb(list: Vec<&str>) -> String {
    let mut proverb: Vec<String> = list
        .iter()
        .zip(list.iter().skip(1))
        .map(|(a, b)| format!("For want of a {} the {} was lost.", a, b))
        .collect();
    
    match list.first() {
        None => return String::new(),
        Some(item) => proverb.push(format!("And all for the want of a {}.", item)),
    }
    proverb.join("\n")
}
