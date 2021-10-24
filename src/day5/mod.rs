use std::time::Instant;
use std::fs;

fn generate() -> Vec<BoardingPass> {
	let contents = fs::read_to_string("input/day5.txt").expect("Failed to load input");
	contents.lines().map(|x| BoardingPass::new(x)).collect()
}

struct BoardingPass {
	pass: String,
	id: usize,
}

impl BoardingPass {
	fn calc_id(&mut self) {
		self.id = self.calc_row(0, 0, 127) * 8 + self.calc_seat(7, 0, 7);
	}
	fn new(pass:&str) -> BoardingPass {
		BoardingPass {pass:pass.to_string(),id:0}
	}

	fn calc_row(&self,i:usize,left: usize, right:usize) -> usize {
		if i == 7 {
			return left;
		}
	
		match self.pass.chars().nth(i).unwrap() {
			'F' => self.calc_row(i + 1, left, left + (right-left) / 2),
			'B' => self.calc_row(i + 1, left + (right-left) / 2 + 1, right),
			_ => 0
		}
	}

	fn calc_seat(&self,i:usize,left: usize, right:usize) -> usize {
		if i == 10 {
			return left;
		}
	
		match self.pass.chars().nth(i).unwrap() {
			'L' => self.calc_seat(i + 1, left, left + (right-left) / 2),
			'R' => self.calc_seat(i + 1, left + (right-left) / 2 + 1, right),
			_ => 0
		}
	}
}


fn part1(input: &mut Vec<BoardingPass>) -> usize {
	let mut biggest_id = 0;
	input.iter_mut().for_each(|x| {
		x.calc_id();
		if x.id > biggest_id { biggest_id = x.id};
	});
	biggest_id
}

fn part2(input: &[BoardingPass]) -> usize {
	let mut ids: Vec<_> = input.iter().map(|f| f.id).collect();
	ids.sort();
	for i in 1..ids.len()-1 {
		if ids[i-1] == ids[i] - 2 {
			return ids[i] - 1; 
		}
	}
	0
}


pub fn run() {
	let now = Instant::now();
	let mut input = generate();
	println!("\tTime to complete generation: {} μs",now.elapsed().as_micros());
	
	let now = Instant::now();
	println!("\tPart 1 result: {}",part1(&mut input));
	println!("\t\tTime to complete part 1: {} μs",now.elapsed().as_micros());
	let now = Instant::now();
	println!("\tPart 2 result: {:?}",part2(&input));
	println!("\t\tTime to complete part 2: {} μs",now.elapsed().as_micros());
}
