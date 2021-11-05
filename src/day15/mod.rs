use std::collections::HashMap;
use std::time::Instant;

fn generate_sequence(starting_sequence: &[usize], n: usize) -> Vec<usize> {
	let mut sequence: Vec<_> = starting_sequence.to_vec();
	let mut unique_elements = HashMap::new();
	for i in 0..starting_sequence.len() - 1 {
		unique_elements.insert(starting_sequence[i], i);
	}
	for i in starting_sequence.len()..n {
		let last_element = sequence[i - 1];
		let num;
		if let Some(index) = unique_elements.get(&last_element) {
			num = i - index - 1;
			*unique_elements.get_mut(&last_element).unwrap() = i - 1;
		} else {
			num = 0;
			unique_elements.insert(last_element, i - 1);
		}
		sequence.push(num);
	}
	sequence
}

fn part1(input: &[usize]) -> usize {
	generate_sequence(input, 2020)[2019]
}

fn part2(input: &[usize]) -> usize {
	generate_sequence(input, 30000000)[30000000 - 1]
}

pub fn run() {
	let now = Instant::now();
	println!(
		"\tTime to complete generation: {} μs",
		now.elapsed().as_micros()
	);

	let now = Instant::now();
	println!("\tPart 1 result: {}", part1(&[16, 11, 15, 0, 1, 7]));
	println!(
		"\t\tTime to complete part 1: {} μs",
		now.elapsed().as_micros()
	);
	let now = Instant::now();
	println!("\tPart 2 result: {}", part2(&[16, 11, 15, 0, 1, 7]));
	println!(
		"\t\tTime to complete part 2: {} μs",
		now.elapsed().as_micros()
	);
}

#[cfg(test)]
mod tests {
	use super::generate_sequence;

	#[test]
	fn test_all_data() {
		assert_eq!(generate_sequence(&[0, 3, 6], 4), [0, 3, 6, 0]);
		assert_eq!(generate_sequence(&[0, 3, 6], 5), [0, 3, 6, 0, 3]);
		assert_eq!(generate_sequence(&[0, 3, 6], 6), [0, 3, 6, 0, 3, 3]);
		assert_eq!(generate_sequence(&[0, 3, 6], 7), [0, 3, 6, 0, 3, 3, 1]);
		assert_eq!(generate_sequence(&[1, 3, 2], 2020)[2019], 1);
		assert_eq!(generate_sequence(&[2, 1, 3], 2020)[2019], 10);
		assert_eq!(generate_sequence(&[1, 2, 3], 2020)[2019], 27);
		assert_eq!(generate_sequence(&[2, 3, 1], 2020)[2019], 78);
		assert_eq!(generate_sequence(&[3, 2, 1], 2020)[2019], 438);
		assert_eq!(generate_sequence(&[3, 1, 2], 2020)[2019], 1836);
	}
	#[test]
	fn test_big_input_data() {
		assert_eq!(
			generate_sequence(&[0, 3, 6], 30000000)[30000000 - 1],
			175594
		);
		assert_eq!(generate_sequence(&[1, 3, 2], 30000000)[30000000 - 1], 2578);
		assert_eq!(
			generate_sequence(&[2, 1, 3], 30000000)[30000000 - 1],
			3544142
		);
		assert_eq!(
			generate_sequence(&[1, 2, 3], 30000000)[30000000 - 1],
			261214
		);
	}
}
