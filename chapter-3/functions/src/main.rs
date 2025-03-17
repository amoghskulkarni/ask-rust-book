fn main() {
    println!("f1 returns: {}", f1());
    f2(5, 'm');
    f3();

    // Compiles, and prints 7
    // Inner block is an expression that returns 6
    // and the block is passed to f4 as an argument
    println!("f4 returns: {}", f4({
        let x = 5;
        x + 1
    }));
}

// Gives compile error: expected `i32`, found `()`
// fn f1() {
//     5
// }
fn f1() -> i32 {
    5
}

fn f2(val: i32, unit: char) {
    println!("The measurement is {val}{unit}");
}

fn f3() {
    println!("This function does not return anything");
}

fn f4(x: i32) -> i32 {
    x + 1
}
