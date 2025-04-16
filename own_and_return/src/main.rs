use own_and_return::*;

fn main() {
    let my_film = Film { name: "Terminator".to_string() };
    println!("{}", take_film_name(my_film));
    // the order of the print statements is intentional, if your implementation is correct,
    // you should have a compile error because my_film was consumed
    let my_films = Film { name: "Terminator".to_string() };
    println!("{}", read_film_name(&my_films));
    // you can test this function by commenting out the first print statement,
    // you should see the expected output without errors in this case
}
