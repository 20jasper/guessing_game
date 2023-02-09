use rand::Rng;
use std::io;

fn main() {
	let secret_number = rand::thread_rng().gen_range(1..=100);

	println!("secret number is {secret_number}");

	println!("Guess a number 1-100");

	let mut guess = String::new();

	io::stdin()
		.read_line(&mut guess)
		.expect("uh oh");

	println!("your guess is {guess}");
}
