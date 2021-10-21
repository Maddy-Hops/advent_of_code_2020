use std::env;
use std::process;

fn main() {
	let args: Vec<String> = env::args().collect();
	if let Err(e) = advent_of_code_2020::run(&args) {
		eprintln!("Application error: {}", e);
		process::exit(1);
	}
}
