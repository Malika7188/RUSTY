pub fn next_prime(nbr: u64) -> u64 {
    let mut next_p = if nbr < 2 {2} else {nbr};
    loop {
        if prime(next_p) {
            return next_p
        }
        next_p += 1
    }
    
}

pub fn prime(n: u64) -> bool {
    if n <= 1 {
        return false
    }
    for i in 2..n {
        if n%i == 0 {
            return false
        }
    }
    true
}