pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        match c {
            '{' | '(' | '[' => stack.push(c),
            '}' | ')' | ']' if stack.is_empty() => return false,
            '}' | ')' | ']' => match (stack.pop().unwrap(), c) {
                ('{', '}') | ('(', ')') | ('[', ']') => (),
                _ => return false,
            },
            _ => (),
        };
    }
    stack.is_empty()
}
