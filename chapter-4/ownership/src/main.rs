fn main() {
    let a = 5;
    let b = a;
    println!("a = {}, b = {}", a, b); // Here a is still valid and can be used

    let s1 = String::from("hello");
    let s2 = s1;
    /* This is different for Strings, because they live in heap. Integer literals
     * are stored in stack, so they are copied. But Strings are stored in heap,
     * so only the reference is copied. The original variable s1 is no longer valid.
     * This is called move.
     */
    // !!! Error !!!
    // println!("s1 = {}, s2 = {}", s1, s2);
    println!("s2 = {}", s2);

    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4); // Both s3 and s4 are valid

    let s5 = String::from("hello");
    takes_ownership(s5);
    /* Similar to earlier example, the ownership of s is moved to the function
     * takes_ownership. So, s is no longer valid here.
     */
    // !!! Error !!!
    // println!("s = {}", s);

    let s6 = String::from("hello");
    let s7 = takes_and_gives_back(s6);
    // !!! Error !!!
    // println!("s6 = {}", s6);
    println!("s7 = {}", s7); // s6 is no longer valid, s7 is valid

    /*
     * Need for references
     *
     * In the above examples, we have seen that ownership is moved when a variable is
     * assigned to another variable. But sometimes we need to pass the variable to a
     * function and still want to use it after the function call.
     *
     * In such cases, we can pass the reference of the variable to the function. This
     * way, the ownership is not moved and the variable is still valid after the function
     * call.
     */
    let s8 = String::from("hello");
    let (s9, len) = calculate_length(s8);
    println!("The length of '{}' is {}", s9, len);
    let len2 = calculate_length_ref(&s9);
    println!("The length of '{}' is {}", s9, len2);
    println!("s9 = {}", s9); // s9 is still valid

    /*
     * Mutable references
     *
     * We can also pass mutable references to the function. This way, the function can
     * modify the variable.
     */
    let mut s10 = String::from("hello");
    change(&mut s10);
    println!("s10 = {}", s10); // s10 is modified by the function

    /*
     * But there are some rules for mutable references:
     * 1. You can have only one mutable reference to a particular piece of data in a
     *    particular scope.
     * 2. You can have multiple immutable references to a particular piece of data in
     *    a particular scope.
     * 3. You cannot have a mutable reference while you have an immutable reference.
     */
    let mut s11 = String::from("hello");
    let r1 = &s11;
    let r2 = &s11;
    // !!! Error !!! -- Rule 3
    // let r3 = &mut s11;
    println!("r1 = {}, r2 = {}", r1, r2);

    let mut s12 = String::from("hello");
    let r1 = &mut s12;
    // !!! Error !!! -- Rule 1
    // let r2 = &mut s12;
    r1.push_str(", world");
    println!("r1 = {}", r1); // r1 is valid as it is the only mutable reference

    /*
     * Dangling references
     *
     * Rust compiler ensures that the references are always valid. It does not allow
     * dangling references. For example, the following code will not compile:
     */
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s
    // }
    // !!! Error !!!
    // let reference_to_nothing = dangle();
    // println!("{}", reference_to_nothing);

    /*
     * Slices
     *
     * Slices are references to a part of a string. They are useful when we want to
     * pass a part of a string to a function. For example:
     */
    let s13 = String::from("hello, world");
    let hello_slice = &s13[0..5];
    let world_slice = &s13[7..12];
    println!(
        "hello_slice = {}, world_slice = {}",
        hello_slice, world_slice
    );
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
