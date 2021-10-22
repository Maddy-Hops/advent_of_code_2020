use std::time::Instant;
use std::fs;

// Assumptions about the input:
// All lowercase
// Required letter always a single char
// all symbols in input are ASCII characters
fn generate() {
	let contents = fs::read_to_string("input/day3.txt").expect("Failed to load input");

}


fn part1() {

}

fn part2(){
	
}


pub fn run() {
	let now = Instant::now();
	let input = generate();
	println!("\tTime to complete generation: {} μs",now.elapsed().as_micros());

	let now = Instant::now();
	println!("\tPart 1 result: {}",part1());
	println!("\t\tTime to complete part 1: {} μs",now.elapsed().as_micros());

	let now = Instant::now();
	println!("\tPart 2 result: {}",part2());
	println!("\t\tTime to complete part 2: {} μs",now.elapsed().as_micros());

}