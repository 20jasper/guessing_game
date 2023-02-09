use std::io;

fn main() {
	println!("Guess a number 1-100");

	let mut guess = String::new();

	io::stdin()
		.read_line(&mut guess)
		.expect("uh oh");

	println!("your guess is {guess}")
}
