/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.trim().is_empty() {
        return false;
    }

    let digits: Vec<_> = isbn.chars().filter(|&c| c != '-').collect();
    if digits.len() != 10 {
        return false;
    }

    let mut sum = 0;

    for (c, n) in digits.into_iter().zip((1..=10).rev()) {
        if c == 'X' {
            sum += 10;
        } else {
            match format!("{}", c).parse::<i32>() {
                Ok(t) => {
                    sum += t * n;
                }
                Err(_) => {
                    return false;
                }
            }
        }
    }

    sum % 11 == 0
}
