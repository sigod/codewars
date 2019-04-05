struct Arith {
	value: &'static str,
}

impl Arith {
	fn string_to_number(value: &str) -> u32 {
		match value {
			"zero" => 0,
			"one" => 1,
			"two" => 2,
			"three" => 3,
			"four" => 4,
			"five" => 5,
			"six" => 6,
			"seven" => 7,
			"eight" => 8,
			"nine" => 9,
			"ten" => 10,
			"eleven" => 11,
			"twelve" => 12,
			"thirteen" => 13,
			"fourteen" => 14,
			"fifteen" => 15,
			"sixteen" => 16,
			"seventeen" => 17,
			"eighteen" => 18,
			"nineteen" => 19,
			"twenty" => 20,
			_ => panic!(),
		}
	}

	fn number_to_string(value: u32) -> &'static str {
		match value {
			0 => "zero",
			1 => "one",
			2 => "two",
			3 => "three",
			4 => "four",
			5 => "five",
			6 => "six",
			7 => "seven",
			8 => "eight",
			9 => "nine",
			10 => "ten",
			11 => "eleven",
			12 => "twelve",
			13 => "thirteen",
			14 => "fourteen",
			15 => "fifteen",
			16 => "sixteen",
			17 => "seventeen",
			18 => "eighteen",
			19 => "nineteen",
			20 => "twenty",
			_ => panic!(),
		}
	}

	fn add(&self, value: &'static str) -> &'static str {
		let a = Arith::string_to_number(self.value);
		let b = Arith::string_to_number(value);

		Arith::number_to_string(a + b)
	}
}

#[test]
fn returns_expected() {
	let c = Arith{value: "three"};
	assert_eq!(c.add("seven"), "ten");
	assert_eq!(c.add("eight"), "eleven");
	assert_eq!(c.add("zero"), "three");
}
