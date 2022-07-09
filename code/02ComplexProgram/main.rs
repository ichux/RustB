use std::io;

fn main() {
	pub mod encryptor;
	pub	mod rot13;
	pub struct Rot13(pub String);

	impl super::Encryptable for Rot13 {
		// add code here
	}
	
	println!("Enter string to encrypt:");

	let mut user_input = String::new();

	io::stdin()
		.read_line(&mut user_input)
		.expect("Cannot read input");

	println!("Your encrypted string: {}", user_input);
}