pub fn abbreviate(phrase: &str) -> String {
    let mut abbr: String = String::new();
    let mut new_word = true;
    let mut prev_upper = false;

    for c in phrase.chars() {
        if c.is_alphanumeric() {
            if new_word {
                abbr.push(c.to_ascii_uppercase());
                new_word = false;
            } else if !prev_upper && c.is_ascii_uppercase() {
                abbr.push(c);
                new_word = false;
            }
        } else {
            new_word = c != '\'';
        }
        prev_upper = c.is_ascii_uppercase();
    }

    abbr
}
