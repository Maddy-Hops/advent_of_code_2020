pub struct Vec2 {
	x: i32,
	y: i32,
}
pub struct Direction {
	direction: Vec2,
}
pub struct Waypoint {
	position: Vec2,
}

pub enum Rotation {
	Left(i32),
	Right(i32),
}
impl Rotation {
	pub fn new(instruction: &str) -> Rotation {
		let val = instruction[1..instruction.len()].parse::<i32>().unwrap() / 90;
		if instruction.chars().nth(0).unwrap() == 'R' {
			if val == 3 {
				return Rotation::Left(1);
			}
			Rotation::Right(val)
		} else {
			if val == 3 {
				return Rotation::Right(1);
			}
			Rotation::Left(val)
		}
	}
}
pub enum Move {
	Forward(i32),
	East(i32),
	South(i32),
	West(i32),
	North(i32),
	Turn(Rotation),
}
pub struct Ship {
	dir: Direction,
	position: Vec2,
	waypoint: Waypoint,
}
impl Ship {
	pub fn new() -> Ship {
		Ship {
			dir: Direction {
				direction: Vec2 { x: 1, y: 0 },
			},
			position: Vec2 { x: 0, y: 0 },
			// is relative to the ship's position
			waypoint: Waypoint {
				position: Vec2 { x: 10, y: -1 },
			},
		}
	}
	pub fn move_ship(&mut self, instruction: &Move) {
		match instruction {
			Move::Forward(val) => {
				self.position.x += self.dir.direction.x * val;
				self.position.y += self.dir.direction.y * val;
			}
			Move::East(val) => {
				self.position.x += val;
			}
			Move::South(val) => {
				self.position.y += val;
			}
			Move::West(val) => {
				self.position.x -= val;
			}
			Move::North(val) => {
				self.position.y -= val;
			}
			Move::Turn(val) => {
				self.dir.turn(val);
			}
		}
	}
	pub fn move_with_waypoint(&mut self, instruction: &Move) {
		match instruction {
			Move::Forward(val) => {
				for _ in 0..*val {
					self.position.x += self.waypoint.position.x;
					self.position.y += self.waypoint.position.y;
				}
			}
			Move::East(val) => {
				self.waypoint.position.x += val;
			}
			Move::South(val) => {
				self.waypoint.position.y += val;
			}
			Move::West(val) => {
				self.waypoint.position.x -= val;
			}
			Move::North(val) => {
				self.waypoint.position.y -= val;
			}
			Move::Turn(val) => {
				self.waypoint.turn(val);
			}
		}
	}
	// returns Manhattan distance
	pub fn manhattan_distance(&self) -> i32 {
		self.position.x.abs() + self.position.y.abs()
	}
}

impl Direction {
	fn turn(&mut self, rot: &Rotation) {
		match rot {
			Rotation::Left(val) => {
				for _ in 0..*val {
					let tmp = self.direction.y;
					self.direction.y = -self.direction.x;
					self.direction.x = tmp;
				}
			}
			Rotation::Right(val) => {
				for _ in 0..*val {
					let tmp = -self.direction.y;
					self.direction.y = self.direction.x;
					self.direction.x = tmp;
				}
			}
		}
	}
}

impl Waypoint {
	fn turn(&mut self, rot: &Rotation) {
		match rot {
			Rotation::Left(val) => {
				for _ in 0..*val {
					let tmp = self.position.y;
					self.position.y = -self.position.x;
					self.position.x = tmp;
				}
			}
			Rotation::Right(val) => {
				for _ in 0..*val {
					let tmp = -self.position.y;
					self.position.y = self.position.x;
					self.position.x = tmp;
				}
			}
		}
	}
}
