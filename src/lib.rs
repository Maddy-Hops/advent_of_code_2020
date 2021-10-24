//! # crate advent_of_code_2020
//! advent_of_code_2020 is a program to run and benchamark solutions to Advent of Code programming puzzles
//! # Syntax for calling the program: 
//! ```
//! ./program_name day1 // where %day1% is the day you want to benchmark
//! ```
//! 
use std::time::Instant;
use std::collections::HashMap;

static DAYS_DONE: usize = 5;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
// mod day6;
// mod day7;
// mod day8;
// mod day9;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;



pub fn run(args: &[String]) -> Result<(),&'static str> {

	let arr = [
		day1::run,
		day2::run,
		day3::run,
		day4::run,
		day5::run,
		];
	if args.len() < 2 {
		return Err("Too few arguments");
	}
	if args[1][3..].parse::<usize>().unwrap() > DAYS_DONE {
		return Err("Day not done yet");
	}
	let mut map = HashMap::new();
	for i in 1..=DAYS_DONE {
		let key = format!("day{}",i);
		map.insert(key,arr[i-1]);
	}
	let now = Instant::now();
	println!("Running {}:",args[1]);
	map.get(&args[1]).unwrap()();
	println!("Total for {}: {} μs",args[1],now.elapsed().as_micros());

	Ok(())
}
