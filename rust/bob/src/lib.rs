pub fn reply(message: &str) -> &str {
    if message.trim().is_empty() {
        "Fine. Be that way!"
    } else if !message.trim().ends_with("?") {
        if message == message.to_uppercase() && message != message.to_lowercase() {
            "Whoa, chill out!"
        } else {
            "Whatever."
        }
    } else {
        if message == message.to_uppercase() && message != message.to_lowercase() {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    }
}
