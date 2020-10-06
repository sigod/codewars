// https://www.codewars.com/kata/5bf71b94e50d1b00590000fe

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn count_squares(lines: &[&str]) -> u64 {
	let space = Space::parse(lines).unwrap();

	let mut count = 0;

	for up_left in space.corners() {
		for up_segment in space.hsegments(&up_left) {
			let length = up_segment.len();

			let flag = space.find_vsegment(up_segment.a, length)
				.and_then(|left_segment| space.find_hsegment(left_segment.b, length))
				.and_then(|_down_segment| space.find_vsegment(up_segment.b, length))
				.is_some();

			if flag {
				count += 1;
			}
		}
	}

	count
}

#[derive(PartialEq)]
enum Cell {
	Empty,
	Corner,
	SideH,
	SideV,
}

impl Cell {
	pub fn parse(symbol: char) -> Result<Self> {
		match symbol {
			' ' => Ok(Self::Empty),
			'+' => Ok(Self::Corner),
			'-' => Ok(Self::SideH),
			'|' => Ok(Self::SideV),
			_ => panic!(),
		}
	}

	pub fn is_valid_h(&self) -> bool {
		*self == Cell::Corner || *self == Cell::SideH
	}

	pub fn is_valid_v(&self) -> bool {
		*self == Cell::Corner || *self == Cell::SideV
	}
}

struct Space {
	space: Vec<Cell>,
	width: usize,
}

impl Space {
	pub fn parse(raw: &[&str]) -> Result<Self> {
		let width = raw.iter()
			.map(|s| s.len())
			.max()
			.unwrap();
		let height = raw.len();
		let mut space = Vec::with_capacity(height * width);

		for line in raw.iter() {
			for c in line.chars() {
				space.push(Cell::parse(c)?);
			}

			for _ in line.len() .. width {
				space.push(Cell::Empty);
			}
		}

		Ok(Self {
			space,
			width,
		})
	}

	pub fn corners<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
		self.space.iter()
			.enumerate()
			.filter_map(move |(index, cell)| {
				if *cell == Cell::Corner {
					Some(Point::from_index(index, self.width))
				}
				else {
					None
				}
			})
	}

	pub fn hsegments<'a>(&'a self, up_left: &'a Point) -> impl Iterator<Item = Segment> + 'a {
		let index = up_left.as_index(self.width);

		self.space.iter()
			.enumerate()
			.skip(index + 1)
			.take(self.width - up_left.x - 1)
			.take_while(|(_, cell)| cell.is_valid_h())
			.filter_map(move |(index, cell)| {
				if *cell == Cell::Corner {
					let up_right = Point::from_index(index, self.width);
					Some(Segment::new(up_left.clone(), up_right))
				}
				else {
					None
				}
			})
	}

	pub fn find_vsegment(&self, a: Point, length: usize) -> Option<Segment> {
		let b = a.step_v(length);

		if !self.is_corner(b) {
			return None;
		}

		let mut flag = true;

		for y in (a.y + 1)..b.y {
			if let Some(cell) = self.space.get(a.set_y(y).as_index(self.width)) {
				if !cell.is_valid_v() {
					flag = false;
					break;
				}
			}
			else {
				flag = false;
				break;
			}

		}

		if flag {
			Some(Segment::new(a, b))
		}
		else {
			None
		}
	}

	pub fn find_hsegment(&self, a: Point, length: usize) -> Option<Segment> {
		let b = a.step_h(length);

		if !self.is_corner(b) {
			return None;
		}

		let index = a.as_index(self.width);

		let flag = self.space.iter()
			.skip(index + 1)
			.take(length - 1)
			.all(|cell| cell.is_valid_h());

		if flag {
			Some(Segment::new(a, b))
		}
		else {
			None
		}
	}

	pub fn is_corner(&self, point: Point) -> bool {
		let index = point.as_index(self.width);

		index < self.space.len()
			&& self.space[index] == Cell::Corner
	}
}

#[derive(Copy, Clone)]
struct Point {
	x: usize,
	y: usize,
}

impl Point {
	pub fn from_index(index: usize, width: usize) -> Self {
		let v = index / width;
		let h = index % width;

		Point {
			x: h,
			y: v,
		}
	}

	pub fn as_index(&self, width: usize) -> usize {
		self.y * width + self.x
	}

	pub fn step_h(&self, length: usize) -> Self {
		Self {
			x: self.x + length,
			y: self.y,
		}
	}

	pub fn step_v(&self, length: usize) -> Self {
		Self {
			x: self.x,
			y: self.y + length,
		}
	}

	pub fn set_y(&self, y: usize) -> Self {
		Self {
			x: self.x,
			y,
		}
	}
}

struct Segment {
	a: Point,
	b: Point,
}

impl Segment {
	pub fn new(a: Point, b: Point) -> Self {
		Self {
			a,
			b,
		}
	}

	pub fn len(&self) -> usize {
		self.b.x - self.a.x + self.b.y - self.a.y
	}
}

//

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_0_example() {
		let lines = vec!(
			"+------+",
			"|      |",
			"+------+",
		);

		assert_eq!(count_squares(&lines), 0);
	}

	#[test]
	fn test_2_example() {
		let lines = vec!(
			"+--+  +----+       ",
			"|  |  |    |    +-+",
			"|  |  +----+    | |",
			"+--+            +-+",
		);

		assert_eq!(count_squares(&lines), 2);
	}

	#[test]
	fn test_3_example() {
		let lines = vec!(
			"+---+  ",
			"|   |  ",
			"| +-+-+",
			"| | | |",
			"+-+-+ |",
			"  |   |",
			"  +---+",
		);

		assert_eq!(count_squares(&lines), 3);
	}

	#[test]
	fn test_5_example() {
		let lines = vec!(
			"+-+-+",
			"| | |",
			"+-+-+",
			"| | |",
			"+-+-+",
		);

		assert_eq!(count_squares(&lines), 5);
	}

	#[test]
	fn test_small_square() {
		let lines = vec!(
			"++",
			"++",
		);

		assert_eq!(count_squares(&lines), 1);
	}

	#[test]
	fn test_unaligned_input() {
		let lines = vec!(
			"",
			"+---+",
		);

		assert_eq!(count_squares(&lines), 0);
	}
}
