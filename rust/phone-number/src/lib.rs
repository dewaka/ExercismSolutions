fn valid_phone_num(phone_num: Vec<char>) -> Option<String> {
    if phone_num[0] > '1' && phone_num[3] > '1' {
        Some(phone_num.iter().collect())
    } else {
        None
    }
}

pub fn number(user_number: &str) -> Option<String> {
    let mut phone_number: Vec<char> = user_number.chars().filter(|c| c.is_digit(10)).collect();

    match phone_number.len() {
        10 => valid_phone_num(phone_number),
        11 => match phone_number.remove(0) {
            '1' => valid_phone_num(phone_number),
            _ => None,
        },
        _ => None,
    }
}
