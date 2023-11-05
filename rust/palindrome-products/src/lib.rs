/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if is_palindrome(format!("{}", value).as_bytes()) {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

fn is_palindrome(s: &[u8]) -> bool {
    let n = s.len();
    for i in 0..n / 2 {
        if s[i] != s[n - 1 - i] {
            return false;
        }
    }
    true
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut pmin: Option<Palindrome> = None;
    let mut pmax: Option<Palindrome> = None;

    for i in min..=max {
        for j in min..=max {
            let m = i * j;

            if let Some(p) = Palindrome::new(m) {
                if pmin.is_none() {
                    pmin = Some(p);
                } else {
                    pmin = Some(Palindrome(pmin.unwrap().into_inner().min(m)));
                }

                if pmax.is_none() {
                    pmax = Some(p);
                } else {
                    pmax = Some(Palindrome(pmax.unwrap().into_inner().max(m)));
                }
            }
        }
    }

    if pmin.is_some() && pmax.is_some() {
        Some((pmin.unwrap(), pmax.unwrap()))
    } else {
        None
    }
}
