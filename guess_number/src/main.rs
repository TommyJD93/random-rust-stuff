use std::io;
use rand::Rng;
use core::cmp::Ordering;

fn generate_random_number() -> u32 {
	return rand::thread_rng().gen_range(1..=10);
}

fn convert_guess(guess: String) -> u32 {
	return  guess.trim().parse().expect("failed to parSe number");
}

fn main() {
    println!("Guess the number game!");

	println!("insert a number: ");

	let mut guess = String::new();

	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line!");
	
	println!("you guessed: {}", guess);	

	let rand = generate_random_number();

	println!("rand: {}", rand);
	let guess = convert_guess(guess);
	
	match guess.cmp(&rand) {
		Ordering::Less => println!("Too Low!"),
		Ordering::Greater => println!("Too High!"),
		Ordering::Equal => println!("You win!"),
	}

}
