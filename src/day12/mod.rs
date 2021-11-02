mod ship_stuff;

use ship_stuff::{Move, Rotation, Ship};
use std::fs;
use std::time::Instant;

fn generate() -> Vec<Move> {
	let contents = fs::read_to_string("input/day12.txt").expect("Failed to read the input file");
	contents
		.lines()
		.map(|f| {
			let instruction = f.chars().nth(0).unwrap();
			let val = f[1..f.len()].parse::<i32>().unwrap();
			match instruction {
				'L' | 'R' => Move::Turn(Rotation::new(f)),
				'F' => Move::Forward(val),
				'E' => Move::East(val),
				'S' => Move::South(val),
				'W' => Move::West(val),
				'N' => Move::North(val),
				_ => panic!("Unknown instruction"),
			}
		})
		.collect()
}

fn part1(input: &[Move]) -> i32 {
	let mut ship = Ship::new();
	for instruction in input {
		ship.move_ship(instruction);
	}
	ship.manhattan_distance()
}

fn part2(input: &[Move]) -> i32 {
	let mut ship = Ship::new();
	for instruction in input {
		ship.move_with_waypoint(instruction);
	}
	ship.manhattan_distance()
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
