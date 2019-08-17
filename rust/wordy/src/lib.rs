use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref QUESTION: Regex = Regex::new(r"^What is (?P<num>-?\d+)(?P<rest>.*)\?$").unwrap();
    static ref OPS: Regex = Regex::new(
        r" (?P<op>plus|minus|multiplied by|divided by|raised to the) (?P<num>-?\d+)(?:\w+ power)?"
    )
    .unwrap();
}

pub fn answer(command: &str) -> Option<i32> {
    if let Some(question) = QUESTION.captures(command) {
        let mut total: i32 = question["num"].parse().ok()?;
        for op in OPS.captures_iter(&question["rest"]) {
            let num: i32 = op["num"].parse().ok()?;
            match &op["op"] {
                "plus" => total += num,
                "minus" => total -= num,
                "multiplied by" => total *= num,
                "divided by" => total /= num,
                "raised to the" => total = total.pow(num as u32),
                _ => return None,
            }
        }

        // regex matches cover the whole command without any extra characters
        if OPS.split(&question["rest"]).collect::<String>().is_empty() {
            return Some(total);
        }
    }
    None
}
