use crate::Command::{Divide, Minus, Multiply, Plus, WhatIs};

#[derive(Debug, Eq, PartialEq)]
enum Command {
    WhatIs(i32),
    Plus(i32),
    Minus(i32),
    Multiply(i32),
    Divide(i32),
}

fn process_commands(commands: &[Command]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];

    for cmd in commands.iter() {
        match cmd {
            WhatIs(x) => stack.push(*x),
            Plus(y) => {
                if let Some(x) = stack.pop() {
                    stack.push(x + y);
                }
            }
            Minus(y) => {
                if let Some(x) = stack.pop() {
                    stack.push(x - y);
                }
            }
            Multiply(y) => {
                if let Some(x) = stack.pop() {
                    stack.push(x * y);
                }
            }
            Divide(y) => {
                if let Some(x) = stack.pop() {
                    stack.push(x / y);
                }
            }
        }
    }

    stack.pop()
}

fn parse_commands(command: &str) -> Option<Vec<Command>> {
    let mut ops = Vec::new();
    let mut command = command;
    if command.ends_with('?') {
        command = &command[0..command.len() - 1]
    }
    let cmds: Vec<_> = command.split(' ').collect();

    let mut i = 0;

    while i < cmds.len() {
        match cmds[i] {
            "What" => {
                if i + 2 >= cmds.len() {
                    return None;
                } else {
                    match cmds[i + 2].parse::<i32>() {
                        Ok(num) => {
                            ops.push(WhatIs(num));
                            i += 3;
                        }
                        Err(_) => return None,
                    }
                }
            }
            "plus" => {
                if i + 1 >= cmds.len() {
                    return None;
                } else {
                    match cmds[i + 1].parse::<i32>() {
                        Ok(num) => {
                            ops.push(Plus(num));
                            i += 2;
                        }
                        Err(_) => return None,
                    }
                }
            }
            "minus" => {
                if i + 1 >= cmds.len() {
                    return None;
                } else {
                    match cmds[i + 1].parse::<i32>() {
                        Ok(num) => {
                            ops.push(Minus(num));
                            i += 2;
                        }
                        Err(_) => return None,
                    }
                }
            }
            "multiplied" => {
                if i + 2 >= cmds.len() {
                    return None;
                } else {
                    match cmds[i + 2].parse::<i32>() {
                        Ok(num) => {
                            ops.push(Multiply(num));
                            i += 3;
                        }
                        Err(_) => return None,
                    }
                }
            }
            "divided" => {
                if i + 2 >= cmds.len() {
                    return None;
                } else {
                    match cmds[i + 2].parse::<i32>() {
                        Ok(num) => {
                            ops.push(Divide(num));
                            i += 3;
                        }
                        Err(_) => return None,
                    }
                }
            }
            _ => return None,
        }
    }

    Some(ops)
}

fn answer_short(command: &str) -> Option<i32> {
    let words: Vec<&str> = command
        .trim_end_matches('?')
        .split_ascii_whitespace()
        .collect();
    match &words[..] {
        &["What", "is", lhs @ _, ref rest @ ..] => {
            let lhs = lhs.parse::<i32>().ok()?;
            parse_operation(lhs, rest)
        }
        _ => None,
    }
}

fn parse_operation(lhs: i32, words: &[&str]) -> Option<i32> {
    match &words[..] {
        &["plus", rhs @ _, ref rest @ ..] => {
            let rhs = rhs.parse::<i32>().ok()?;
            parse_operation(lhs + rhs, rest)
        }
        &["minus", rhs @ _, ref rest @ ..] => {
            let rhs = rhs.parse::<i32>().ok()?;
            parse_operation(lhs - rhs, rest)
        }
        &["multiplied", "by", rhs @ _, ref rest @ ..] => {
            let rhs = rhs.parse::<i32>().ok()?;
            parse_operation(lhs * rhs, rest)
        }
        &["divided", "by", rhs @ _, ref rest @ ..] => {
            let rhs = rhs.parse::<i32>().ok()?;
            parse_operation(lhs / rhs, rest)
        }
        &[] => Some(lhs),
        &[..] => None,
    }
}

fn answer_original(command: &str) -> Option<i32> {
    parse_commands(command).and_then(|ops| process_commands(&ops))
}

pub fn answer(command: &str) -> Option<i32> {
    // answer_original(command)
    answer_short(command)
}

#[cfg(test)]
mod test {
    use crate::Command::{Divide, Minus, Multiply, Plus, WhatIs};
    use crate::{parse_commands, process_commands};

    #[test]
    fn test_process_commands() {
        assert_eq!(Some(7), process_commands(&[WhatIs(3), Plus(4)]));
        assert_eq!(
            Some(49),
            process_commands(&[WhatIs(3), Plus(4), Multiply(7)])
        );
        assert_eq!(Some(25), process_commands(&[WhatIs(3), Plus(4), Plus(18)]));
        assert_eq!(Some(3), process_commands(&[WhatIs(12), Divide(4)]));
        assert_eq!(Some(5), process_commands(&[WhatIs(7), Minus(2)]));
    }

    #[test]
    fn test_parse_commands() {
        assert_eq!(
            Some(vec![WhatIs(-3), Plus(7), Multiply(-2)]),
            parse_commands("What is -3 plus 7 multiplied by -2?")
        );

        assert_eq!(
            Some(vec![WhatIs(-3), Multiply(25)]),
            parse_commands("What is -3 multiplied by 25?")
        );

        assert_eq!(
            Some(vec![WhatIs(2), Multiply(-2), Multiply(3)]),
            parse_commands("What is 2 multiplied by -2 multiplied by 3?")
        );
    }
}
