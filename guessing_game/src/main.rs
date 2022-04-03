use std::io; /* std is the standard library and io input/output library */
// checkout the doc to know more about std => https://doc.rust-lang.org/std/prelude/index.html

fn main() {
    println!("Guess the number");

    println!("Please input your guess");

    let mut guess = String::new(); // Declare a mutable variable to stored the user input

    // Read the user input and store it in the guess variable
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess); // {} is a placeholder for the parameter, I can also use numbers or names to identify the parameters
}
