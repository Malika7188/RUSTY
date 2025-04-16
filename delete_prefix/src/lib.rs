// Instructions
// Define the function delete_prefix which returns the string slice s with the prefix removed. It should be wrapped in Some. If prefix is not a prefix of s, then delete_prefix returns None.

// Expected Function
// pub fn delete_prefix(prefix: &str, s: &str) -> Option<&str> {
// }
// Usage
// Here is a program to test your function.

// use delete_prefix::delete_prefix;

// fn main() {
// 	println!("{:?}", delete_prefix("ab", "abcdefghijklmnop"));
// 	println!("{:?}", delete_prefix("x", "abcdefghijklmnop"));
// }
// And its output:

// $ cargo run
// Some("cdefghijklmnop")
// None
// $
pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str> {
    if s.starts_with(prefix) {
        Some(&s[prefix.len()..])
    } else {
        None
    }
}
