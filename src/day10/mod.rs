use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn generate() -> Vec<i32> {
	let contents = fs::read_to_string("input/day10.txt").expect("Failed to read the input file");
	let mut input: Vec<i32> = contents
		.lines()
		.map(|f| f.parse::<i32>().unwrap())
		.collect();
	input.sort();
	// let's add the 0 and the max_adapter + 3 links to our list
	input.reverse();
	input.push(0);
	input.reverse();
	input.push(input[input.len() - 1] + 3);
	input
}

// since the requirements state we must find the chain including **all** adapters
// we're going to walk through the sorted list and just record the differences between elements
fn part1(input: &[i32]) -> i32 {
	let mut map = HashMap::new();
	for i in 0..input.len() - 1 {
		let count = map.entry(input[i + 1] - input[i]).or_insert(0);
		*count += 1;
	}
	map.get(&1).unwrap() * map.get(&3).unwrap()
}

fn part2(input: &[i32]) -> u128 {
	// finding all the combinations of connecting your device
	// first we need to find a 3-wide gap, that means there's only one valid way to connect two groups of combinations
	// we can then calculate the combinations for each group in-between 3-wide gaps separately and multiply them together to get the final answer
	let mut mult = 1;
	let mut prev_start = 0;
	let mut all_slices = vec![];
	for i in 0..input.len() - 1 {
		if input[i + 1] - input[i] == 3 {
			let slice = &input[prev_start..i + 1];
			all_slices.push(slice.into_iter().map(|&f| f).collect::<Vec<i32>>());
			prev_start = i + 1;
		}
	}
	// each resulting slice will consist of elements separated by one and
	// the number of possible paths it has will be following a fibonacci-like
	// sequence with 3 previous numbers considered, starting with `0,1,1`
	// like so:
	// series = [0,1,1,2,4,7,13,24,44]
	// n's (zero indexed) element in the series will be the number of combinations in this particular slice
	// we go through all the slices, discard anything that's below 3 in length (because there's only 1 way to go there and
	// there's no point in multiplying by 1 a bunch of times)
	// then we search for the longest slice in our list of slices, and generate the sequence up to that element
	// then we simply go over each slice, look at their length and multply our result by series[len(slice)]
	all_slices.retain(|f| f.len() >= 3);
	let mut longest_slice = 0;
	all_slices.iter().for_each(|f| {
		if f.len() > longest_slice {
			longest_slice = f.len()
		}
	});
	let mut series = vec![0, 1, 1];
	for i in 2..=longest_slice {
		series.push(series[i - 2] + series[i - 1] + series[i])
	}
	all_slices.iter().for_each(|f| mult *= series[f.len()]);
	mult
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
