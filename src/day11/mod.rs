use std::fs;
use std::time::Instant;

fn generate() -> Vec<Vec<Seat>> {
	let contents = fs::read_to_string("input/day11.txt").expect("Failed to read the input file");
	let mut input: Vec<String> = contents.lines().map(|f| f.to_string()).collect();
	input.iter_mut().for_each(|f| {
		f.push('.');
		f.insert_str(0, ".");
	});
	input.push(".".repeat(input[0].len()));
	input.reverse();
	input.push(".".repeat(input[0].len()));
	input.reverse();
	input
		.into_iter()
		.map(|f| {
			f.chars()
				.into_iter()
				.map(|x| match x {
					'#' => Seat::Occ,
					'L' => Seat::Emp,
					'.' => Seat::Flr,
					_ => panic!("Unknown symbol in input"),
				})
				.collect()
		})
		.collect()
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Seat {
	Occ, // occupied
	Emp, // empty
	Flr, // floor
}

// If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
// If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
// Otherwise, the seat's state does not change.

fn part1(input: &Vec<Vec<Seat>>) -> i32 {
	let mut field_before = input.clone();
	let mut field_new = field_before.clone();
	loop {
		let mut has_changed = false;
		for i in 1..field_before.len() - 1 {
			for j in 1..field_before[0].len() - 1 {
				match check_neighbours(&field_before, i, j) {
					(counter, Seat::Emp) if counter == 0 => {
						field_new[i][j] = Seat::Occ;
						has_changed = true;
					}
					(counter, Seat::Occ) if counter >= 4 => {
						field_new[i][j] = Seat::Emp;
						has_changed = true;
					}
					_ => (),
				}
			}
		}
		field_before = field_new.clone();
		if !has_changed {
			break;
		}
	}
	let field_new: Vec<Seat> = field_new.into_iter().flat_map(|f| f).collect();
	let mut counter = 0;
	field_new.into_iter().for_each(|f| {
		if f == Seat::Occ {
			counter += 1
		}
	});
	counter
}
const ARR: [(i32, i32); 8] = [
	(1, 1),
	(1, 0),
	(0, 1),
	(-1, -1),
	(-1, 0),
	(0, -1),
	(1, -1),
	(-1, 1),
];
fn check_neighbours(field: &Vec<Vec<Seat>>, i: usize, j: usize) -> (usize, Seat) {
	let mut counter = 0;

	for offset in ARR {
		if field[(i as i32 + offset.0) as usize][(j as i32 + offset.1) as usize] == Seat::Occ {
			counter += 1;
		}
	}
	(counter, field[i][j].clone())
}

fn part2(input: Vec<Vec<Seat>>) -> i32 {
	let mut field_before = input;
	let mut field_new = field_before.clone();
	loop {
		let mut has_changed = false;
		for i in 1..field_before.len() {
			for j in 1..field_before[0].len() {
				match check_neighbours_in_line(&field_before, i, j) {
					(counter, Seat::Emp) if counter == 0 => {
						field_new[i][j] = Seat::Occ;
						has_changed = true;
					}
					(counter, Seat::Occ) if counter >= 5 => {
						field_new[i][j] = Seat::Emp;
						has_changed = true;
					}
					_ => (),
				}
			}
		}
		field_before = field_new.clone();
		if !has_changed {
			break;
		}
	}
	let field_new: Vec<Seat> = field_new.into_iter().flat_map(|f| f).collect();
	let mut counter = 0;
	field_new.into_iter().for_each(|f| {
		if f == Seat::Occ {
			counter += 1
		}
	});
	counter
}
fn check_neighbours_in_line(field: &Vec<Vec<Seat>>, i: usize, j: usize) -> (usize, Seat) {
	let mut counter = 0;

	for offset in ARR {
		let (mut loop_i, mut loop_j) = (i as i32, j as i32);
		while loop_i >= 1
			&& loop_j >= 1
			&& (loop_i as usize) < field.len() - 1
			&& (loop_j as usize) < field[0].len() - 1
		{
			loop_i += offset.0;
			loop_j += offset.1;
			if field[loop_i as usize][loop_j as usize] == Seat::Occ {
				counter += 1;
				break;
			} else if field[loop_i as usize][loop_j as usize] == Seat::Emp {
				break;
			}
		}
	}
	(counter, field[i][j].clone())
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
	println!("\tPart 2 result: {:?}", part2(input));
	println!(
		"\t\tTime to complete part 2: {} μs",
		now.elapsed().as_micros()
	);
}

#[cfg(test)]
mod tests {
	fn test_rules(input: Vec<String>) {}
}
