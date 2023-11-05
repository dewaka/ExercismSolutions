use std::collections::HashSet;

fn normalize(word: &str) -> Vec<char> {
    let mut chars: Vec<char> = word.to_lowercase().chars().collect();
    chars.sort();
    chars
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut res: HashSet<&'a str> = HashSet::new();
    let given = normalize(word);

    for &p in possible_anagrams {
        let candidate = normalize(p.clone());
        if candidate == given && p.to_lowercase() != word.to_lowercase() {
            res.insert(p);
        }
    }

    res
}
