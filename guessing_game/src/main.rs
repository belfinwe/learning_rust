use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {

        println!("Please input your guess.");

        // let declare variable
        // mut, makes the variable mutable (immutable as default in Rust!)
        // String, as in Java? new is a constructor function in the String type (class?) 
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)  // & indicates that the argument is a reference
            .expect("Failed to read line");  // Exception?

        // trim() removes white spaces at the beginning and end of the string
        // parse() converts the string to another type
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
