pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                let a = b'a';
                let offset = (c as u8 - a + ((key % 26 + 26) % 26) as u8) % 26;
                (a + offset) as char
            } else if c.is_ascii_uppercase() {
                let a = b'A';
                let offset = (c as u8 - a + ((key % 26 + 26) % 26) as u8) % 26;
                (a + offset) as char
            } else {
                c
            }
        })
        .collect()
}
