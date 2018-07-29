fn yelling(message: &str) -> bool {
    return message.to_uppercase() == message && message.to_lowercase() != message;
}

fn is_question(message: &str) -> bool {
    return message.ends_with('?');
}

pub fn reply(message: &str) -> &str {
    let trimmed = message.trim_right();
    if yelling(trimmed) && is_question(trimmed) {
        return "Calm down, I know what I'm doing!";
    }
    if is_question(trimmed) {
        return "Sure.";
    }
    if yelling(trimmed) {
        return "Whoa, chill out!";
    }
    if trimmed == "" {
        return "Fine. Be that way!";
    }
    return "Whatever.";
}
