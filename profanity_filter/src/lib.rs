pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.is_empty() || message.contains("stupid") {
        return Err("ERROR: illegal")
    } else {
        return Ok(message)
    }
}