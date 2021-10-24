use std::time::Instant;
use std::fs;
use std::collections::HashMap;

fn generate() {
	let contents = fs::read_to_string("input/day4.txt").expect("Failed to load input");
	contents.split("\n\n").map(|x| PassportEntry::new(x)).collect::<Vec<_>>()
}


fn part1() {
	
}


pub fn run() {
	let now = Instant::now();
	println!("\tTime to complete generation: {} μs",now.elapsed().as_micros());

	let now = Instant::now();
	//println!("\tPart 1 result: {:?}",counter);
	println!("\t\tTime to complete part 1: {} μs",now.elapsed().as_micros());

	let now = Instant::now();
	//println!("\tPart 2 result: {:?}",part2(valid_entries));
	println!("\t\tTime to complete part 2: {} μs",now.elapsed().as_micros());
}
