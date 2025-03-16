fn main() {
    // Throws errror because of explicit types not given
    // let fourty_two = "42".parse().expect("Not a number!");

    let fourty_two: u32 = "42".parse().expect("Not a number!"); // Comiples, parses string "42" to number

    /**
     * Scalar types
     */
    let f: f32 = 3.0; // f32 type
    let i: i32 = 3; // i32 type
    let b: bool = true; // bool type
    let c: char = 'z'; // char type
    let s: &str = "Hello, world!"; // &str type

    /**
     * Compound types: Tuple and Array
     */
    let t: (i32, f64, u8) = (42, 6.12, 1); // Tuple type
    let (x, y, z) = t; // Destructuring tuple
    println!("x: {x}, y: {y}, z: {z}");
    println!("First element of tuple: {t.0}");

    let a = [1, 2, 3, 4, 5]; // Array type
    let first = a[0]; // Accessing array element
    println!("First element of array: {first}");

    let a = [3; 5]; // Array type with 5 elements of value 3
    
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // Slicing array
    println!("Slice: {:?}", slice);

    /**
     * String type and operations
     */
    let s = String::from("Hello, world!"); // String type
    let s = "Hello, world!".to_string(); // String type
    let s = String::from("Hello, ") + "world!"; // String concatenation
    let s = format!("Hello, {}", "world!"); // String formatting
    let s = "Hello, world!".to_string();
    let s = s + " Goodbye!"; // String concatenation
    let s = "Hello, world!";
    let s = s.replace("world", "there"); // String replacement
    println!("{s}");


}
