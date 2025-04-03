use yyy::*;
fn main() {
    // Test the collatz function
    println!("Testing collatz with input 3:");
    collatz(3);
    
    println!("\nTesting print_bytes with input \"Déjà Vu\\n\":");
    print_bytes("Déjà Vu\n");
    
    // Uncommenting this will run the yes function which runs forever
    // println!("\nTesting yes function (will run forever):");
    // yes();
}