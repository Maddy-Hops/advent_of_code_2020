use std::time::Instant;
use std::fs;

fn generate() -> Vec<i32> {
	let contents = fs::read_to_string("input/day1.txt").expect("Failed to load input");
	contents.lines().map(|x|x.parse().unwrap()).collect()
}


fn part1_optimized(input: &[i32]) -> i32 {
	let max = input.len();
	let mut j_start: usize = 1;
	for i in 0..max {
		for j in j_start..max-1 {
			if input[i] + input[j] == 2020 {
				return input[i] * input[j];
			}
		}
		j_start += 1;
	}
	return -1;
}

fn part2_optimized(input: &[i32]) -> i32 {
	let max = input.len();
	let mut j_start: usize = 1;
	let mut k_start: usize = 2;
	for i in 0..max-2 {
		for j in j_start..max-1 {
			for k in k_start..max {
				if input[i] + input[j] + input[k] == 2020 {
					return input[i] * input[j] * input[k];
				}
			}
		}
		j_start += 1;
		k_start += 2;

	}
	return -1;
}

pub fn run() {
	let now = Instant::now();
	let input = generate();
	println!("\tTime to complete generation: {} μs",now.elapsed().as_micros());

	let now = Instant::now();
	println!("\tPart 1 result: {}",part1_optimized(&input));
	println!("\t\tTime to complete part 1: {} μs",now.elapsed().as_micros());

	let now = Instant::now();
	println!("\tPart 2 result: {}",part2_optimized(&input));
	println!("\t\tTime to complete part 2: {} μs",now.elapsed().as_micros());

}