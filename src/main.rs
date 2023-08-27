use clap::Parser;
use rand::Rng;

/// Password generator
fn generate_password(length: usize, mut complexity: usize) -> String {
	let alphabeth: [&str; 4] = [
		"abcdefghijklmnopqrstuvwxyz",
		"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
		"0123456789",
		"!@#$%^&*()-_=+[]{};:,.<>?",
	];
	if complexity > alphabeth.len() {complexity = alphabeth.len()};
	let result_alphabeth = &alphabeth[..complexity].concat();
	let mut result = String::with_capacity(length);
	// Create random generator
	let mut rng = rand::thread_rng();
	for _ in 0..length {
		let index: usize = rng.gen_range(0 as usize..result_alphabeth.len());
		if let Some(ch) = result_alphabeth.chars().nth(index) {
			result.push(ch);
		}
	}
	result
}

/// Simple programm for generate password.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Password length
	#[arg(short, long, default_value_t = 25)]
	length: usize,

	/// Password complexity
	#[arg(short, long, default_value_t = 4)]
	count: usize,
}

fn main() {
	let args = Args::parse();
	let pass = generate_password(args.length, args.count);
	println!("{}", pass);
}