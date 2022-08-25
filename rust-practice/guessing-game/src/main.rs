use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // let guess will make a variable as well, but default variables are IMMUTABLE
    // we add "mut" to make this string mutable for user input.
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
