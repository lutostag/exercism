fn yelling(message: &str) -> bool {
    message.to_uppercase() == message && message.to_lowercase() != message
}

fn question(message: &str) -> bool {
    message.ends_with('?')
}

pub fn reply(message: &str) -> &str {
    let message = message.trim_right();

    if yelling(message) && question(message) {
        "Calm down, I know what I'm doing!"
    } else if yelling(message) {
        "Whoa, chill out!"
    } else if question(message) {
        "Sure."
    } else if message.is_empty() {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }
}
