pub fn lucas_number(n: u32) -> u32 {

    if n == 0 {
        return 2
    } else if n == 1 {
        return 1
    }
    let mut sum = 0;

    let mut a = 2;
    let mut b = 1;

    for _ in 2..n+1 {
        sum = a+b;
        a = b;
        b = sum;
    }   
    sum
}