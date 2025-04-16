pub fn count_factorial_steps(factorial: u64) -> u64 {
    if factorial <= 1 {
        return 0;
    }
    let mut steps = 1;
    let mut product = 1;

    loop {
        product *= steps;
        if product == factorial {
            return steps;
        } else if product > factorial {
            return 0
        }
        steps += 1
    }
}