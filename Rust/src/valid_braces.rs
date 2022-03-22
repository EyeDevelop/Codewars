pub fn valid_braces(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    s.chars().map(|c| match c {
        '(' => {stack.push(')'); true}
        '[' => {stack.push(']'); true}
        '{' => {stack.push('}'); true}
        ')' => stack.pop().unwrap_or(' ') == ')',
        ']' => stack.pop().unwrap_or(' ') == ']',
        '}' => stack.pop().unwrap_or(' ') == '}',
        _ => false,
    }).all(|b| b) && stack.len() == 0
}