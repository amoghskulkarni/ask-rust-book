const TEST_CONST: &str = "Mad skillz bruh";

fn mut_eg() {
    let x = 5;
    println!("x: {x}");
    // x = 6; // Invalid, rustc gives error E0384
    let x = 6; // Creates a new variable to "shadow" previous x
    println!("x: {x}");
}

fn print_const() {
    println!("TEST_CONST: {TEST_CONST}")
}

fn main() {
    // mut_eg();
    // print_const();
}
