pub fn is_anagram(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false
    }

    let mut c1: Vec<char> = s1.to_lowercase().chars().collect();
    let mut c2: Vec<char> = s2.to_lowercase().chars().collect();

    c1.sort_unstable();
    c2.sort_unstable();

    c1 == c2
}