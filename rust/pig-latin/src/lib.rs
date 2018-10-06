static VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];

pub fn translate_word(input: &str) -> String {
    let mut front = String::from(input);

    if front.starts_with("yt") || front.starts_with("xr") {
        return front + "ay";
    }

    let split = front.find(VOWELS).or(front.find('y'));
    let split = split.unwrap_or(0);
    let mut back: String = front.drain(..split).collect();

    if back.ends_with("q") && front.starts_with("u") {
        back.push(front.remove(0));
    }
    front + &back + "ay"
}

pub fn translate(input: &str) -> String {
    input
        .split(" ")
        .map(translate_word)
        .collect::<Vec<String>>()
        .join(" ")
}
