use std::io::stdin;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // let secret_number = rand::thread_rng().gen_range(1..=100);
    let secret_number: u32 = rand::rng().random_range(1..=100);
    println!("Secret Number: {secret_number}");

    println!("Guess the number!");

    loop {
        println!("Please input your guess. Press Q to quit.");
    
        let mut guess = String::new();
    
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // This will only print the error message, but won't execute anything else
        // to handle the error. We need to use match for that.
        // let guess: u32 = guess.trim().parse::<u32>().expect("Please enter a number!");
        
        // This will let us quit the game by typing a string that is unparsable 
        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                // If you continue, the game goes on
                // All errors are ignored 
                // continue;

                println!("Quitting..");
                break;
            },
        };
    
        // Another way that string formatting can be done
        // println!("You guessed: {guess}");
        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            }
        }
    
        
    }

}