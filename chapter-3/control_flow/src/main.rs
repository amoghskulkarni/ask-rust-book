fn main() {
    /*
     If-else expressions
     If-else expressions are used to make decisions based on conditions.
     */
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if expressions with else if
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement
    let condition = true;
    // Gives an error because the types are different
    // let number = if condition { 5 } else { "six" };
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // Using if in a let statement a little differently
    // Following code is equivalent to the above code
    let condition = true;
    let number;
    if condition {
        number = 5;
    } else {
        number = 6;
    }
    println!("The value of number is: {}", number);
    

    /*
     Loop
     Loops are used to execute a block of code repeatedly until a condition is met.
    */

    // Breaking loop and returning a value from a loop 
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Loop labels
    // Loop labels are used to break out of a specific loop when there are nested loops.
    // Q: What does following code snippet do?
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");


    /*
     While loop
     While loop is used to execute a block of code repeatedly as long as a condition is true.
    */
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // Using while loop to iterate over an array using array length
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    /*
     For loop
     For loop is used to iterate over a collection of items.
    */
    // Using for loop to iterate over an array is better 
    // than using while loop because it is less error-prone.
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {element}");
    }
}
