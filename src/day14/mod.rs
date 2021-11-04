use std::collections::HashMap;
use std::time::Instant;
use std::{char, fs};

#[derive(Clone, Copy, Debug)]
struct Bitmask {
	val: [BitmaskChars; 36],
}

impl Bitmask {
	fn new(mask: &str) -> Self {
		let mut val: [BitmaskChars; 36] = [BitmaskChars::X; 36];
		let mut i = 0;
		mask.chars().for_each(|f| {
			match f {
				'X' => val[i] = BitmaskChars::X,
				_ => val[i] = BitmaskChars::Digit(f),
			}
			i += 1;
		});
		Self { val }
	}
}
#[derive(Clone, Copy, Debug)]
enum BitmaskChars {
	Digit(char),
	X,
}
#[derive(Clone, Debug)]
struct MemCommand {
	key: usize,
	val: [char; 36],
}

impl MemCommand {
	fn new(line: &str) -> Self {
		let mut key = String::new();
		let mut val = String::new();
		let mut flag_write_key = false;
		let mut flag_write_val = false;
		for char in line.chars() {
			match char {
				']' => flag_write_key = false,
				'\n' => flag_write_val = false,
				_ => (),
			}
			if flag_write_key {
				key.push(char);
			}
			if flag_write_val {
				val.push(char);
			}
			match char {
				'[' => flag_write_key = true,
				'=' => flag_write_val = true,
				_ => (),
			}
		}

		let val = usize_to_binary(val.trim().parse::<usize>().unwrap());
		Self {
			key: key.trim().parse().unwrap(),
			val,
		}
	}
	fn write_to_memory_part1(
		&self,
		memory_object: &mut HashMap<usize, [char; 36]>,
		bitmask: &Bitmask,
	) {
		let mut final_value: [char; 36] = self.val.clone();
		for i in 0..bitmask.val.len() {
			if let BitmaskChars::Digit(dgt) = bitmask.val[i] {
				final_value[i] = dgt;
			}
		}
		memory_object.insert(self.key, final_value);
	}
	fn write_to_memory_part2(
		&self,
		memory_object: &mut HashMap<usize, [char; 36]>,
		bitmask: &Bitmask,
	) {
		let addresses_to_change = generate_all_addresses(&bitmask, &usize_to_binary(self.key));
		for address in addresses_to_change {
			memory_object.insert(binary_to_usize(address), self.val);
		}
	}
}
#[derive(Clone, Debug)]
enum Instruction {
	Mask(Bitmask),
	MemWrite(MemCommand),
}
fn generate() -> Vec<Instruction> {
	let contents = fs::read_to_string("input/day14.txt").expect("Failed to read the input file");
	let mut input = vec![];
	for line in contents.lines() {
		if line.starts_with("mask") {
			input.push(Instruction::Mask(Bitmask::new(&line[7..line.len()])));
		} else {
			input.push(Instruction::MemWrite(MemCommand::new(line)));
		}
	}
	input
}

fn part1(input: &[Instruction]) -> usize {
	let mut bitmask = Bitmask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
	let mut memory_object = HashMap::new();
	for instruction in input {
		match instruction {
			Instruction::Mask(mask) => bitmask = *mask,
			Instruction::MemWrite(mem_command) => {
				mem_command.write_to_memory_part1(&mut memory_object, &bitmask)
			}
		}
	}
	let mut sum = 0;
	memory_object
		.values()
		.for_each(|f| sum += binary_to_usize(*f));
	sum
}

fn usize_to_binary(val: usize) -> [char; 36] {
	let binary_representation = format!("{:#038b}", val);
	let binary_representation =
		(&binary_representation[2..binary_representation.len()]).to_string();
	let mut val = ['0'; 36];
	let mut i = 0;
	binary_representation.chars().for_each(|f| {
		val[i] = f;
		i += 1;
	});
	val
}

fn binary_to_usize(char_arr: [char; 36]) -> usize {
	let mut result = String::new();
	for char in char_arr {
		result.push(char);
	}
	usize::from_str_radix(&result, 2).unwrap()
}
fn generate_all_addresses(bitmask: &Bitmask, original_address: &[char; 36]) -> Vec<[char; 36]> {
	let mut addresses_to_change = vec![];
	let mut address = ['0'; 36];
	for i in 0..bitmask.val.len() {
		match bitmask.val[i] {
			BitmaskChars::Digit(ch) if ch == '1' => address[i] = '1',
			BitmaskChars::Digit(ch) if ch == '0' => address[i] = original_address[i],
			BitmaskChars::X => address[i] = 'X',
			_ => panic!("Unknown symbol"),
		}
	}
	addresses_to_change.push(address);
	for i in 0..bitmask.val.len() {
		for address in addresses_to_change.clone().iter() {
			if address[i] == 'X' {
				let mut address_with_1 = address.clone();
				let mut address_with_0 = address.clone();
				address_with_1[i] = '1';
				address_with_0[i] = '0';
				addresses_to_change.push(address_with_0);
				addresses_to_change.push(address_with_1);
			}
		}
	}
	addresses_to_change.retain(|f| !f.contains(&'X'));
	addresses_to_change
}

fn part2(input: &[Instruction]) -> usize {
	let mut bitmask = Bitmask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
	let mut memory_object = HashMap::new();
	for instruction in input {
		match instruction {
			Instruction::Mask(mask) => bitmask = *mask,
			Instruction::MemWrite(mem_command) => {
				mem_command.write_to_memory_part2(&mut memory_object, &bitmask)
			}
		}
	}
	let mut sum = 0;
	memory_object
		.values()
		.for_each(|f| sum += binary_to_usize(*f));
	sum
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
mod tests {
	use crate::day14::{binary_to_usize, usize_to_binary};

	#[test]
	fn convesion_to_usize() {
		assert_eq!(
			binary_to_usize([
				'0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0',
				'0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0',
				'1', '0', '1', '1'
			]),
			11_usize
		);
	}
	#[test]
	fn convesion_to_binary() {
		assert_eq!(
			usize_to_binary(11),
			[
				'0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0',
				'0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0',
				'1', '0', '1', '1'
			]
		);
	}
}
