
use std::time::Instant;




mod day1;
mod day2;
// mod day3;
// mod day4;
// mod day5;
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
	if args.len() < 2 {
		return Err("Too few arguments");
	}

	
	if args[1] == "day1" {
		let now = Instant::now();
		println!("Running day 1:");
		day1::run();
		println!("Total for day 1: {} μs",now.elapsed().as_micros());
	}


	if args[1] == "day2" {
		let now = Instant::now();
		println!("Running day 2:");
		day2::run();
		println!("Total for day 2: {} μs",now.elapsed().as_micros());
	}

	Ok(())
}
