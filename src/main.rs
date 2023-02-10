use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
	let secret_number = rand::thread_rng().gen_range(1..=100);

	println!("secret number is {secret_number}");

	loop {
		println!("Guess a number 1-100");

		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("uh oh");

		let guess: u32 = guess
			.trim()
			.parse()
			.expect("Please type a number!");

		println!("your guess is {guess}");

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		};
	}
}
