use std::time::Instant;
use std::fs;
use std::collections::HashMap;

fn generate<'a>(contents: &'a str) -> Vec<PassportEntry<'a>> {
	contents.split("\n\n").map(|x| PassportEntry::new(x)).collect::<Vec<_>>()
}

const FIELDS: &'static [&'static str ] = &[
	"byr",
	"iyr",
	"eyr",
	"hgt",
	"hcl",
	"ecl",
	"pid",
	"cid"];

#[allow(dead_code)]
struct PassportEntry<'a> {
	fields: HashMap<&'a str,&'a str>,
	valid_no_cid: bool,
	valid: bool,
}

impl<'a> PassportEntry<'a> {
	// Assumptions about the input:
	// every input string is a series of key:value pairs separated by comma
	// every input string is valid ASCII
	fn new(input: &'a str) -> PassportEntry<'a> {
		let input = input.split('\n')
		.flat_map(|x| x.split_whitespace())
		.collect::<Vec<_>>();
		let keys_values: Vec<_> = input.into_iter().map(|f| f.split(":")).collect();
		let mut fields = HashMap::new();
		keys_values.into_iter()
		.map(|mut f| fields.insert(f.next().unwrap(), f.next().unwrap()))
		.for_each(drop);
		
		let mut valid = true;
		let mut valid_no_cid = true;
		for &provided_field in FIELDS {
			if !fields.contains_key(provided_field) {
				if provided_field != "cid" {
					valid = false;
					valid_no_cid = false;
				} else {
					valid = false;
				}
			}
		}
		PassportEntry {
			fields,
			valid,
			valid_no_cid,
		}
	}

	fn valid_data(&self) -> bool {
		// validate byr
		if let Ok(byr) = self.fields["byr"].parse::<i32>() {
			if byr < 1920 || byr > 2002 {
				return false;
			}
		} else {
			return false;
		}
		// validate iyr
		if let Ok(iyr) = self.fields["iyr"].parse::<i32>() {
			if iyr < 2010 || iyr > 2020 {
				return false;
			}
		} else {
			return false;
		}
		// validate eyr
		if let Ok(eyr) = self.fields["eyr"].parse::<i32>() {
			if eyr < 2020 || eyr > 2030 {
				return false;
			}
		} else {
			return false;
		}
		// validate hgt
		if let Some(hgt_in) = self.fields["hgt"].strip_suffix("in") {
			if let Ok(hgt) = hgt_in.parse::<i32>() {
				if hgt > 76 || hgt < 59 {
					return false;
				}
			} else {
				return false;
			}
		} else if let Some(hgt_cm) = self.fields["hgt"].strip_suffix("cm") {
			if let Ok(hgt) = hgt_cm.parse::<i32>() {
				if hgt > 193 || hgt < 150 {
					return false;
				}
			} else {
				return false;
			}
		} else {
			return false;
		}
		// validate hcl
		if let Some(hcl) = self.fields["hcl"].strip_prefix("#") {
			if hcl.len() != 6 {
				return false;
			} else {
				for i in 0..=5 {
					if !hcl.chars().nth(i).unwrap().is_ascii_hexdigit() {
						return false;
					}
				}
			}
		} else {
			return false;
		}
		// validate ecl
		let ecl = self.fields["ecl"];
		if !(ecl == "amb" 
		|| ecl == "blu"
		|| ecl == "brn"
		|| ecl == "gry"
		|| ecl == "grn"
		|| ecl == "hzl" 
		|| ecl == "oth") {
			return false;
		}
		// validate pid
		if self.fields["pid"].len() == 9 {
			if let Ok(_pid) = self.fields["pid"].parse::<i32>() {
				return true;
			} else {
				return false;
			}
		} else {
			return false;
		}
	}
}



fn part1<'a>(input: &'a [PassportEntry]) -> (usize,Vec<&'a PassportEntry<'a>>) {
	let mut valid_entries = vec![];
	for entry in input {
		if entry.valid_no_cid {
			valid_entries.push(entry.clone());
		}
	}
	(valid_entries.len(),valid_entries)
}

// valid entries has all fields
fn part2<'a>(valid_entries: Vec<&'a PassportEntry>) -> usize {
	let mut counter = 0;
	for entry in valid_entries {
		if entry.valid_data() {
			counter += 1;
		}
	}
	counter
}


pub fn run() {
	let now = Instant::now();
	let contents = fs::read_to_string("input/day4.txt").expect("Failed to load input");
	let input = generate(&contents);
	println!("\tTime to complete generation: {} μs",now.elapsed().as_micros());

	let now = Instant::now();
	let (counter,valid_entries) = part1(&input);
	println!("\tPart 1 result: {:?}",counter);
	println!("\t\tTime to complete part 1: {} μs",now.elapsed().as_micros());

	let now = Instant::now();
	println!("\tPart 2 result: {:?}",part2(valid_entries));
	println!("\t\tTime to complete part 2: {} μs",now.elapsed().as_micros());
}
