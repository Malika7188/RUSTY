fn main() {
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizbuz")
        } else if n%3 == 0 {
            println!("fiz")
        } else if n%5 == 0 {
            println!("buzz")
        } else if n%11 == 3 {
            println!("FIZZ")
        } else if n%11 == 5 {
            println!("BUZZ")
        } else {
            println!("{}", n)
        }

    }

}
