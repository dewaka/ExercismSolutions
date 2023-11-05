use crate::AffineCipherError::NotCoprime;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b > 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn is_coprime(a: i32, b: i32) -> bool {
    if ((a | b) & 1) != 1 {
        // both `a` and `b` are even
        return false;
    }
    1 == gcd(a, b)
}

fn check_coprime(a: i32, b: i32) -> Result<(), AffineCipherError> {
    if is_coprime(a, b) {
        Ok(())
    } else {
        Err(NotCoprime(a))
    }
}

fn apply_alphabetic<F>(c: char, f: F) -> char
where
    F: Fn(i32) -> i32,
{
    if c.is_numeric() {
        c
    } else {
        let x = char_to_numeric(c);
        let v = f(x);
        numeric_to_char(v)
    }
}

const ASCII_OFFSET: u8 = b'a';

fn char_to_numeric(c: char) -> i32 {
    ((c as u8) - ASCII_OFFSET) as i32
}

fn numeric_to_char(x: i32) -> char {
    char::from_u32((x + ASCII_OFFSET as i32) as u32).unwrap()
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    check_coprime(a, 26).map(|_| {
        plaintext
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| apply_alphabetic(c, |x| (a * x + b) % 26))
            .collect::<Vec<char>>()
            .chunks(5)
            .map(|word| word.into_iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    })
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    check_coprime(a, 26).map(|_| mmi(a, 26)).map(|mmi| {
        ciphertext
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| apply_alphabetic(c, |x| ((mmi * (x - b) % 26 + 26) % 26)))
            .collect::<String>()
    })
}

fn mmi(a: i32, m: i32) -> i32 {
    (2..m).find(|n| a * n % m == 1).unwrap()
}

#[cfg(test)]
mod test {
    use crate::is_coprime;

    #[test]
    fn test() {
        assert!(!is_coprime(12, 14));
        assert!(is_coprime(13, 5));
        assert!(!is_coprime(20, 5));
    }
}
