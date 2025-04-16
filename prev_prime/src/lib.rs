pub fn prev_prime(nbr: u64) -> u64  {
    for i in (2..nbr).rev() {
        if prime(i) {
            return i
        }
    }
    0
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