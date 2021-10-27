use std::convert::TryInto;
use std::time::Instant;
use std::fs;
use std::collections::HashMap;

fn generate() -> Vec<(String,String)> {
	let contents = fs::read_to_string("input/day8.txt").expect("Failed to read input/day8.txt");
	contents.lines()
	.map(|f| {
		let mut iter = f.split_ascii_whitespace();
		(iter.next().unwrap().to_string(),iter.next().unwrap().to_string())
	}).collect()
	//.map(|x| x.to_string()).collect())
	
}

struct CommandRunner {
	acc: i128,
	current_line: usize,
	list_of_visited: Vec<usize>,
}

impl CommandRunner {
	fn run(command_list: &Vec<(String,String)>) -> (i128,Result<(),()>) {
		let mut result: (i128,Result<(),()>) = (0,Err(()));
		let mut cr = CommandRunner::new();
		loop {

			if cr.list_of_visited.contains(&cr.current_line) {
				result.1 = Err(());
				break;
			} else if cr.current_line >= command_list.len() {
				println!("Program terminated normally");
				result.1 = Ok(());
				break;
			}
			cr.list_of_visited.push(cr.current_line);
			match &command_list[cr.current_line] {
				(command, val) if command == "acc" => {
					cr.acc += val.parse::<i128>().unwrap();
					
					cr.current_line += 1;
				},
				(command, _val) if command == "nop" => {
					cr.current_line += 1;
				},
				(command, val) if command == "jmp" => {
					let mut jmp_val = val.parse::<i32>().unwrap();
					if jmp_val >= 0 {
						let jmp_val: usize = jmp_val.try_into().unwrap();
						cr.current_line += jmp_val;
					} else {
						jmp_val = -jmp_val;
						let jmp_val: usize = jmp_val.try_into().unwrap();
						cr.current_line -= jmp_val;
					}
					
				},
				_ => panic!("Encountered an unknown command"),
			}
		}
		result.0 = cr.acc;
		result
	}
	
	fn new() -> CommandRunner {
		CommandRunner {
			acc: 0, current_line: 0, list_of_visited: vec![]
		}
	}

}

fn part1(command_list: &Vec<(String,String)>) -> i128 {
	CommandRunner::run(command_list).0
}



fn part2(command_list: &mut Vec<(String,String)>) -> i128 {
	let mut map = HashMap::new();
	map.insert("jmp", "nop");
	map.insert("nop", "jmp");
	let mut result: (i128,Result<(),()>) = (0,Err(()));
	let mut has_changed = false;
	for i in 0..command_list.len() {
		// change the list back if loop ran again
		if has_changed {
			if command_list[i-1].0 == "nop" || command_list[i-1].0 == "jmp" {
				command_list[i-1].0 = map.get(&command_list[i-1].0 as &str).unwrap().to_string();
				has_changed = false;
			}
		}
		// change the next command
		let command = &command_list[i].0;
		if command_list[i].0 == "nop" || command_list[i].0 == "jmp" {
			command_list[i].0 = map.get(&command as &str).unwrap().to_string();
			has_changed = true;
		}
		 
		result = CommandRunner::run(&command_list);
		if result.1 == Ok(()) {
			break;
		}
	}
	result.0
}

pub fn run() {
	let now = Instant::now();
	
	let mut input = generate();
	println!("\tTime to complete generation: {} μs",now.elapsed().as_micros());
	
	let now = Instant::now();
	println!("\tPart 1 result: {}",part1(&input));
	println!("\t\tTime to complete part 1: {} μs",now.elapsed().as_micros());
	let now = Instant::now();
	println!("\tPart 2 result: {:?}",part2(&mut input));
	println!("\t\tTime to complete part 2: {} μs",now.elapsed().as_micros());
}