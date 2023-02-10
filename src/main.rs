use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
	let secret_number = rand::thread_rng().gen_range(1..=100);

	println!("secret number is {secret_number}");

	loop {
		println!("Guess a number 1-100");

		let mut guess = String::new();

		match io::stdin().read_line(&mut guess) {
			Ok(str) => str,
			Err(_) => {
				println!("error reading line, try again");
				continue;
			}
		};

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("error parsing number, please try again");
				continue;
			}
		};

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
