use std::time::Instant;
use std::collections::HashMap;
use std::{fs, vec};

fn generate<'a>(contents: &'a str) -> HashMap<String,Bag> {
	// first go through the list rules to generate every bag color
	// then go through a second time filling in the `contains` field
	let mut input: HashMap<String,Bag> =HashMap::new();
	let mut colors = vec![];

	for line in contents.lines() {
		let rules:Vec<_> = line.split_ascii_whitespace().map(|x|x.to_string()).collect();
		let color = Bag::set_color(&rules, 0);
		colors.push(color.clone());
		let bag = Bag::new(&line);
		input.insert(color, bag);
	}
	
	for i in 0..input.len() {
		input.get_mut(&colors[i]).unwrap().set_contains();
	}

	input
}


#[derive(PartialEq, Eq,Debug,Clone)]
struct Bag<'a> {
	contained_in: Vec<&'a Bag<'a>>,
	contains: HashMap<String,usize>,
	color: String,
	rules: Vec<String>,
}

impl<'a> Bag<'a> {
	fn new(rules: &str) -> Bag{
		let rules:Vec<_> = rules.split_ascii_whitespace().map(|x|x.to_string()).collect();
		let color = Bag::set_color(&rules, 0);
		let contains = HashMap::new();
		let contained_in = vec![];
		Bag{ contained_in, contains, color, rules }
	}

	fn set_contains(&mut self) { //, mut bags: HashMap<String,Bag>) {
		// if there's no bags contained inside of this one just leave the field empty
		if self.rules[4] == "no" {
			return;
		} else {
			for i in (4..self.rules.len()).step_by(4) {
				let color = Bag::set_color(&self.rules, i+1);
				self.contains.insert(color.clone(), self.rules[i].parse::<usize>().unwrap());
			}
		}
	}

	fn set_color(rules: &[String],i: usize) -> String {
		let mut color = vec![];
		color.push(rules[i].clone());
		color.push(rules[i+1].clone());
		color.join(" ")
	}
}

fn part1(input: &mut HashMap<String,Bag>,bag_to_search: &str) -> usize {
	
	let mut vec_of_colors = vec![];
	for (color,bag) in input.iter() {
		bag.contains.keys().for_each(|x| if x == bag_to_search {vec_of_colors.push(color.clone())} );
	}
	let mut has_changed = true;
	let mut colors_last_iter = vec_of_colors.clone();
	while has_changed {
		let mut colors_vec = vec![];
		has_changed = false;
		for i in colors_last_iter.iter() {
			for (color,bag) in input.iter() {
				bag.contains.keys().for_each(|x| if x == i {colors_vec.push(color.clone())} );
			}
		}
		colors_last_iter = colors_vec.clone();
		if colors_vec.len() > 0 {
			colors_vec.into_iter().for_each(|x| vec_of_colors.push(x));
			has_changed = true;
		}
	}
	vec_of_colors.sort();
	vec_of_colors.dedup();
	vec_of_colors.len()
}

fn recursive_go_into_bag (bag: &Bag,input: &HashMap<String,Bag>)-> usize {
	let contains = &input.get(&bag.color).unwrap().contains;
	let mut counter:usize = 0;
	contains.into_iter().for_each(|(k,v)|{
		counter += v;
		counter += v * recursive_go_into_bag(input.get(k).unwrap(),&input);
	});
	counter
}

fn part2(input: &HashMap<String,Bag>,bag_to_search: &str) -> usize {
	recursive_go_into_bag(input.get(bag_to_search).unwrap(), &input)
}

pub fn run() {
	let bag_to_search = "shiny gold";
	let now = Instant::now();
	let contents = fs::read_to_string("input/day7.txt").expect("Failed to read input/day7.txt");
	let mut input = generate(&contents);
	println!("\tTime to complete generation: {} μs",now.elapsed().as_micros());
	
	let now = Instant::now();
	println!("\tPart 1 result: {}",part1(&mut input,bag_to_search));
	println!("\t\tTime to complete part 1: {} μs",now.elapsed().as_micros());
	let now = Instant::now();
	println!("\tPart 2 result: {:?}",part2(&input,bag_to_search));
	println!("\t\tTime to complete part 2: {} μs",now.elapsed().as_micros());
}