pub fn yes(){
    let mut count = 0;
    loop {
        println!("y");
        count += 1;
        if count >= 5 {
            break;
        }
    }
}
pub fn collatz(start: u32) {
    let mut n = start;
    println!("{}", n);

    while n != 1 {
        if n%2 == 0 {
            n /= 2
        } else {
            n = 3 * n + 1;
        }
        println!("{}", n);

    }
}
pub fn print_bytes(s: &str) {
    for b in s.bytes() {
        println!("{}", b)
    }
}
