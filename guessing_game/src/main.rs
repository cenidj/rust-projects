use std::io; /* std is the standard library and io input/output library */
// checkout the doc to know more about std => https://doc.rust-lang.org/std/prelude/index.html
use rand::Rng; /* rand is the random number library and Rng is the random number generator */
use std::cmp::Ordering; /* cmp is the comparison library */

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new(); // Declare a mutable variable to stored the user input

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

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
