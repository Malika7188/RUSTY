pub fn invert_sentence(string: &str) -> String {
    // Split into words while preserving spaces
    let mut result = String::new();
    let mut word = String::new();

    // Collect all characters in reverse to manually control word placement
    for ch in string.chars().rev() {
        if ch.is_whitespace() {
            // If we were building a word, push it reversed (correct order) to the result
            if !word.is_empty() {
                result.push_str(&word.chars().rev().collect::<String>());
                word.clear();
            }
            result.push(ch); // keep the whitespace
        } else {
            word.push(ch); // build the word in reverse
        }
    }

    // Last word (if any)
    if !word.is_empty() {
        result.push_str(&word.chars().rev().collect::<String>());
    }

    result
}
