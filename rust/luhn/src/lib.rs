/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.trim();
    if code.len() <= 1 {
        return false;
    }
    let mut sum = 0;

    for (p, c) in code
        .chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .enumerate()
    {
        match format!("{}", c).parse::<i32>() {
            Ok(n) => {
                sum += if p % 2 == 1 {
                    let n = n * 2;
                    if n > 9 {
                        n - 9
                    } else {
                        n
                    }
                } else {
                    n
                }
            }
            Err(_) => return false,
        }
    }

    sum % 10 == 0
}
