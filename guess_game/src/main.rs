
use std::io;

fn main() {
    println!("Guess the Number");

    println!("Please input your nuomber:");

//. In Rust, variables are immutable by default,
// meaning once we give the variable a value, the value won’t change


    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)  //(&) references are immutable by defaultl
    .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
