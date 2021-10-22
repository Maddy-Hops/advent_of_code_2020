use std::time::Instant;
use std::fs;

// Assumptions about the input:
// All lowercase
// Required letter always a single char
// all symbols in input are ASCII characters
fn generate() -> Vec<DbEntry> {
	let contents = fs::read_to_string("input/day2.txt").expect("Failed to load input");
	let contents: Vec<_> = contents.lines().collect();
	let mut input: Vec<DbEntry> = vec![];
	for line in contents {
		input.push(DbEntry::new(line));
	}
	return input;
}

struct DbEntry {
	req_letter: char,
	min: usize,
	max: usize,
	password: String,
}

impl DbEntry {
	// Constructs a new sturctured DbEntry from String Line of input
	fn new(line: &str) -> DbEntry {
		let req_letter: char;
		let min: usize;
		let max: usize;
		let password: String;
		// [0] is min-max | [1] is req letter | [2] is password entry
		let line: Vec<_> = line.split_ascii_whitespace().collect();
		req_letter = line[1].chars().nth(0).unwrap();
		password = line[2].to_string();
		let min_max: Vec<usize> = line[0].split("-").map(|x| x.parse().unwrap()).collect();
		min = min_max[0];
		max = min_max[1];
		DbEntry{
			req_letter,
			min,
			max,
			password,
		}
	}
}

fn part1(input: &[DbEntry]) -> usize {
	let mut count_valid = 0;
	for entry in input {
		let count_req_letters = entry.password.matches(entry.req_letter).count();
		if count_req_letters >= entry.min && count_req_letters <= entry.max {
			count_valid += 1;
		}
	}
	return count_valid;
}

fn part2(input: &[DbEntry]) -> usize {
	let mut count_valid = 0;
	for entry in input {
		let mut letters_in_the_right_place = 0;
		if entry.password.chars().nth(entry.min-1).unwrap() == entry.req_letter {
			letters_in_the_right_place += 1;
		}
		if entry.password.chars().nth(entry.max-1).unwrap() == entry.req_letter {
			letters_in_the_right_place += 1;
		}
		if letters_in_the_right_place == 1 {
			count_valid += 1;
		}
	}
	return count_valid;
}


pub fn run() {
	let now = Instant::now();
	let input = generate();
	println!("\tTime to complete generation: {} μs",now.elapsed().as_micros());

	let now = Instant::now();
	println!("\tPart 1 result: {}",part1(&input));
	println!("\t\tTime to complete part 1: {} μs",now.elapsed().as_micros());

	let now = Instant::now();
	println!("\tPart 2 result: {}",part2(&input));
	println!("\t\tTime to complete part 2: {} μs",now.elapsed().as_micros());

}