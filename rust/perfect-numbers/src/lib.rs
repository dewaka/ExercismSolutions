use crate::Classification::{Abundant, Deficient, Perfect};

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    match (1..=num / 2).filter(|n| num % n == 0).sum::<u64>() {
        _ if num == 0 => None,
        s if s > num => Some(Abundant),
        s if s < num => Some(Deficient),
        _ => Some(Perfect),
    }
}
