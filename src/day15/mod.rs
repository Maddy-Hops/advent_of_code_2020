use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn generate() -> Vec<i32> {
	let contents = fs::read_to_string("input/day15.txt").expect("Failed to read the input file");
	vec![]
}

fn part1(input: &[i32]) -> usize {
	0
}

fn part2(input: &[i32]) -> usize {
	0
}

pub fn run() {
	let now = Instant::now();
	let input = generate();
	println!(
		"\tTime to complete generation: {} μs",
		now.elapsed().as_micros()
	);

	let now = Instant::now();
	println!("\tPart 1 result: {}", part1(&input));
	println!(
		"\t\tTime to complete part 1: {} μs",
		now.elapsed().as_micros()
	);
	let now = Instant::now();
	println!("\tPart 2 result: {:?}", part2(&input));
	println!(
		"\t\tTime to complete part 2: {} μs",
		now.elapsed().as_micros()
	);
}

#[cfg(test)]
mod tests {}
