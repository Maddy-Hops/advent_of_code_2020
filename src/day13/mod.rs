use inline_python::{python, Context};
use std::fs;
use std::time::Instant;
fn generate() -> (Schedule, Vec<ScheduleEntry>) {
	let contents = fs::read_to_string("input/day13.txt").expect("Failed to read the input file");
	let mut iter = contents.lines();
	let timestamp_arrival = iter.next().unwrap().parse().unwrap();
	let mut bus_list = vec![];
	let mut bus_list2 = vec![];
	iter.next().unwrap().split(",").for_each(|f| {
		if f != "x" {
			let val = f.parse::<i32>().unwrap();
			bus_list.push(val);
			bus_list2.push(ScheduleEntry::Bus(val as u128));
		} else {
			bus_list2.push(ScheduleEntry::Emp);
		}
	});

	(
		Schedule {
			timestamp_arrival,
			bus_list,
		},
		bus_list2,
	)
}
struct Schedule {
	timestamp_arrival: i32,
	bus_list: Vec<i32>,
}

#[derive(Debug)]
enum ScheduleEntry {
	Bus(u128),
	Emp,
}

fn part1(input: &Schedule) -> i32 {
	let mut earliest_bus_timestamp = vec![];
	let bus_list = input.bus_list.clone();
	for bus in bus_list.clone() {
		earliest_bus_timestamp
			.push(input.timestamp_arrival - (input.timestamp_arrival % bus) + bus);
	}
	let mut zipped_timestamp_bus_vec: Vec<(i32, i32)> =
		earliest_bus_timestamp.into_iter().zip(bus_list).collect();
	zipped_timestamp_bus_vec.sort();
	(zipped_timestamp_bus_vec[0].0 - input.timestamp_arrival) * zipped_timestamp_bus_vec[0].1
}

fn part2(input: &[ScheduleEntry]) -> u128 {
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
	println!("\tPart 1 result: {}", part1(&input.0));
	println!(
		"\t\tTime to complete part 1: {} μs",
		now.elapsed().as_micros()
	);
	let now = Instant::now();
	println!("\tPart 2 result: {:?}", part2(&input.1));
	println!(
		"\t\tTime to complete part 2: {} μs",
		now.elapsed().as_micros()
	);
}
