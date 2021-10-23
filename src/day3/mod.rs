use std::ops::Add;
use std::time::Instant;
use std::fs;

fn generate() -> Vec<String> {
	let contents = fs::read_to_string("input/day3.txt").expect("Failed to load input");
	let mut input: Vec<String> = vec![];
	for line in contents.lines() {
		input.push(line.to_string());
	}
	input
}


struct Vec2 {
	x: usize,
	y: usize,
}

impl<'a, 'b> Add<&'b Vec2> for &'a Vec2 {
	type Output = Vec2;
	fn add(self, other_vec: &'b Vec2) -> Self::Output {
		Vec2 {x:self.x + other_vec.x, y: self.y + other_vec.y} 
	}
}


struct SlopeMap {
	limit: usize,
	current_position: Vec2,

}
impl SlopeMap {
	fn new(input: Vec<String>) -> SlopeMap {
		SlopeMap {
			limit:input[0].len() - 1,
			current_position:Vec2 { x: 0, y: 0 }
		}
	}
	fn set_position(&mut self, pos: &Vec2) {
		let mut x = pos.x;
		let y = pos.y;
		if pos.x > self.limit {
			x = pos.x - self.limit - 1;
		}
		self.current_position.x = x;
		self.current_position.y = y;
	}
}

fn part1(input: &[String]) -> u32 {
	let offset = Vec2{x:3,y:1};
	let mut map = SlopeMap::new(input.to_vec());
	let mut counter = 0;
	for row in input {		
		if row.chars().nth(map.current_position.x).unwrap() == '#' {
			counter += 1;
		}
		map.set_position(&(&(map.current_position) + &offset))
	}
	counter
}

fn part2(input: &[String]) -> u128 {
	let offsets = [
		Vec2{x:1,y:1},
		Vec2{x:3,y:1},
		Vec2{x:5,y:1},
		Vec2{x:7,y:1},
		Vec2{x:1,y:2},
	];
	let mut map = SlopeMap::new(input.to_vec());
	let mut result:u128 = 1;
	let mut skip_row = false;
	for offset in offsets {
		let mut counter = 0;
		for row in input {
			if skip_row {
				skip_row = !skip_row;
				continue;
			}
			if offset.y == 2 {
				skip_row = !skip_row;
			}
			if row.chars().nth(map.current_position.x).unwrap() == '#' {
				counter += 1;
			}
			map.set_position(&(&(map.current_position) + &offset));
		}
		result *= counter;
		map.set_position(&Vec2{x:0,y:0});
	}
	result
}


pub fn run() {
	let now = Instant::now();
	let input = generate();
	println!("\tTime to complete generation: {} μs",now.elapsed().as_micros());

	let now = Instant::now();
	println!("\tPart 1 result: {:?}",part1(&input));
	println!("\t\tTime to complete part 1: {} μs",now.elapsed().as_micros());

	let now = Instant::now();
	println!("\tPart 2 result: {:?}",part2(&input));
	println!("\t\tTime to complete part 2: {} μs",now.elapsed().as_micros());
}
