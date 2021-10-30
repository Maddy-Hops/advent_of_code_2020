use std::time::Instant;
use std::fs;
use std::collections::VecDeque;

fn generate() -> VecDeque<i128> {
	let contents = fs::read_to_string("input/day9.txt").expect("Failed to read the input file");
	contents.lines()
	.map(|f| f.parse::<i128>().unwrap()).collect()
	
}


fn part1(mut input: VecDeque<i128>) -> i128 {
	let preamble_len = 25;
	let mut preamble = VecDeque::new();
	for _ in 0..preamble_len {
		preamble.push_back(input.pop_front());
	}
	loop {
		let val = input.pop_front().unwrap();
		let mut is_valid = false;
		for i in  0..preamble_len {
			for j in i+1..preamble_len {
				if val == preamble[i].unwrap() + preamble[j].unwrap() {
					is_valid = true;
				}
			}
		}
		if !is_valid {
			return val;
		}
		preamble.pop_front();
		preamble.push_back(Some(val));
	}
}



fn part2(input: VecDeque<i128>, result_part1: i128) -> i128 {
	let mut contiguous_set = vec![];
	for i in 0..input.len() {
		contiguous_set.clear();
		contiguous_set.push(input[i]);
		let mut current_sum = input[i];
		for j in i+1..input.len() {
			contiguous_set.push(input[j]);
			current_sum += input[j];
			if current_sum > result_part1 {
				break;
			} else if current_sum == result_part1 {
				contiguous_set.sort();
				return contiguous_set[0] + contiguous_set.pop().unwrap();
			}
		}
	}
	0
}

pub fn run() {
	let now = Instant::now();
	
	let input = generate();
	println!("\tTime to complete generation: {} μs",now.elapsed().as_micros());
	
	let now = Instant::now();
	let result_part1 = part1(input.clone());
	println!("\tPart 1 result: {}",&result_part1);
	println!("\t\tTime to complete part 1: {} μs",now.elapsed().as_micros());
	let now = Instant::now();
	println!("\tPart 2 result: {:?}",part2(input,result_part1));
	println!("\t\tTime to complete part 2: {} μs",now.elapsed().as_micros());
}