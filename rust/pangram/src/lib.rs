use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let cset: HashSet<_> = sentence
        .chars()
        .filter(|c| c.is_alphabetic() && c.is_ascii())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    cset.len() == 26
}
