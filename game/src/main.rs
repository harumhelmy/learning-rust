use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // the ampersand indicates that the arg here is a reference
            .expect("Failed to read line");

        let secret_number = rand::thread_rng().gen_range(1, 101);

        // reusing the variable name, as below, is also known as shadowing
        // more on that later, apparently
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
