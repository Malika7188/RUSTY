pub fn min(a: i32, b: i32) -> i32 {
    if a == b {
        return 0
    }
    if a > b {
        return b;
    } else {
        return a;
    }
    
}