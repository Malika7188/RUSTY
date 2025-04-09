pub fn reverse_it(v: i32) -> String {
    // let abs_str: String = v.abs().to_string().chars().rev().collect();
    let str_v = v.to_string();
    let abs_str = str_v.trim_start_matches('-');
    let rev: String = abs_str.chars().rev().collect();
    if v < 0 {
        format!("-{}{}", rev, abs_str)
    } else {
        format!("{}{}", rev, abs_str)
    }
}