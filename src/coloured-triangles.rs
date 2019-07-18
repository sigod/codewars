fn triangle(row_str: &str) -> String {
	let mut array = row_str.chars().collect::<Vec<char>>();

	for iteration in 1..array.len() {
		for index in 0..(array.len() - iteration) {
			array[index] = next_color(array[index], array[index + 1]);
		}
	}

	format!("{}", array[0])
}

fn next_color(left: char, right: char) -> char {
	match (left, right) {
		('R', 'R') => 'R',
		('G', 'G') => 'G',
		('B', 'B') => 'B',

		('B', 'G') => 'R',
		('G', 'B') => 'R',

		('B', 'R') => 'G',
		('R', 'B') => 'G',

		('R', 'G') => 'B',
		('G', 'R') => 'B',

		(_, _) => panic!(),
	}
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_triangle() {
		assert_eq!(triangle("GB"), "R");
		assert_eq!(triangle("RRR"), "R");
		assert_eq!(triangle("RGBG"), "B");
		assert_eq!(triangle("RBRGBRB"), "G");
		assert_eq!(triangle("RBRGBRBGGRRRBGBBBGG"), "G");
		assert_eq!(triangle("GB"), "R");
		assert_eq!(triangle("B"), "B");
	}
}
