pub fn lastup(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let lower = word.to_lowercase();
            let mut chars: Vec<char> = lower.chars().collect();

            if let Some(last) = chars.last_mut() {
                last.make_ascii_uppercase();
            }
            chars.into_iter().collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}