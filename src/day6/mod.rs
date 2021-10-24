use std::time::Instant;
use std::fs;
use std::collections::HashMap;

fn generate(contents: &str) ->  Vec<Vec<&str>> {
	//let mut groups = vec![];
	
	contents.split("\n\n").map(|x|{
		 let mut tmp = x.split("\n").collect::<Vec<&str>>();
		 tmp.sort();
		 tmp
		}).collect()
	
}



fn part1(input: &Vec<Vec<&str>>) -> usize {
	let input = input.clone();
	let mut groups: Vec<Vec<char>> = vec![];
	for group in input {
		let mut grp = vec![];
		for person in group {
			grp.push(person.chars().collect());
		}
		groups.push(grp.into_iter().flat_map(|f: Vec<_>| f).collect());
	}
	let mut count = 0;
	for mut group in groups {
		group.sort();
		group.dedup();
		count += group.len();
	}
	count
}

fn part2(input: &mut Vec<Vec<&str>>) -> usize {
	let mut count = 0;
	for group in input {
		let num_people = group.len();
		if num_people == 1 {
			count += group[0].len();
		} else {
			let mut map = HashMap::new();
			for person in group {
				for i in 0..person.len() {
					let count = map.entry(person.chars().nth(i).unwrap()).or_insert(0);
					*count += 1;
				}
			}
			map.retain(|&_k,v|  *v == num_people);
			count += map.len();
		}
	}
	count
}


pub fn run() {
	let now = Instant::now();
	let contents = fs::read_to_string("input/day6.txt").expect("Failed to load input");
	let mut input = generate(&contents);
	println!("\tTime to complete generation: {} μs",now.elapsed().as_micros());

	let now = Instant::now();
	println!("\tPart 1 result: {}",part1(&input));
	println!("\t\tTime to complete part 1: {} μs",now.elapsed().as_micros());
	let now = Instant::now();
	println!("\tPart 2 result: {:?}",part2(&mut input));
	println!("\t\tTime to complete part 2: {} μs",now.elapsed().as_micros());
}
