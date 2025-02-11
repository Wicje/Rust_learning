
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Guess the Number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    

    loop {

        println!("Please input Your guess");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering ::Greater => println!("Too great"),
            Ordering::Equal => {
                 println!("You win");
                 break;
            }
        }

    }
}




//a crate is a collection of Rust source code files.