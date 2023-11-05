fn left_bracket(c: char) -> char {
    match c {
        '}' => '{',
        ')' => '(',
        ']' => '[',
        _ => panic!("Not a bracket: `{}`", c),
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::with_capacity(string.len());

    for c in string.chars() {
        match c {
            '{' | '(' | '[' => stack.push(c),
            '}' | ')' | ']' => match stack.pop() {
                Some(left) => {
                    if left != left_bracket(c) {
                        return false;
                    }
                }
                None => return false,
            },
            _ => (),
        }
    }

    stack.is_empty()
}
