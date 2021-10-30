use std::time::Instant;
use std::fs;

fn generate() -> Vec<i32>{
	let contents = fs::read_to_string("input/day12.txt").expect("Failed to read the input file");
	contents.lines()
	.map(|f| f.parse::<i32>().unwrap()).collect()
	
}


fn part1(input: &[i32]) -> i32 {
	0
}



fn part2(input: &[i32]) -> i32 {
	
	0
}

pub fn run() {
	let now = Instant::now();
	
	let input = generate();
	println!("\tTime to complete generation: {} μs",now.elapsed().as_micros());
	
	let now = Instant::now();
	println!("\tPart 1 result: {}",part1(&input));
	println!("\t\tTime to complete part 1: {} μs",now.elapsed().as_micros());
	let now = Instant::now();
	println!("\tPart 2 result: {:?}",part2(&input));
	println!("\t\tTime to complete part 2: {} μs",now.elapsed().as_micros());
}