use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
	let random_num = rand::thread_rng().gen_range(1..=100);

	println!("random num is {random_num}");

	loop {
		println!("guess a number 1 - 100");

		let mut guess = String::new();

		match io::stdin().read_line(&mut guess) {
			Ok(str) => str,
			Err(_) => {
				println!("error reading line");
				continue;
			}
		};

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				print!("invalid number");
				continue;
			}
		};

		println!("your guess is {guess}");

		match guess.cmp(&random_num) {
			Ordering::Less => println!("too low"),
			Ordering::Greater => println!("too big"),
			Ordering::Equal => {
				println!("you win");
				break;
			}
		};
	}
}
