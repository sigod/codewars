fn is_sum_of_cubes_divisible(chunk: &str) -> bool {
	let sum = chunk
		.chars()
		.map(|c| c.to_digit(10).unwrap())
		.fold(0, |acc, digit| acc + digit.pow(3));

	sum % 2 == 0
}

fn revrot(string: &str, chunk_size: usize) -> String {
	if chunk_size == 0 || string.len() < chunk_size {
		return "".to_string();
	}

	let mut left_index = 0;
	let mut right_index = chunk_size;
	let mut result = String::with_capacity(string.len());

	while right_index <= string.len() {
		let chunk = &string[left_index..right_index];

		if is_sum_of_cubes_divisible(&chunk) {
			for c in chunk.chars().rev() {
				result.push(c);
			}
		}
		else {
			result.push_str(&chunk[1..]);
			result.push(chunk.chars().next().unwrap());
		}

		left_index += chunk_size;
		right_index += chunk_size;
	}

	result
}

fn testing(s: &str, c: usize, exp: &str) -> () {
	assert_eq!(&revrot(s, c), exp)
}

#[test]
fn basics_revrot() {
	testing("1234", 0, "");
	testing("", 0, "");
	testing("1234", 5, "");
	let s = "733049910872815764";
	testing(s, 5, "330479108928157");
	let s = "73304991087281576455176044327690580265896";
	testing(s, 8, "1994033775182780067155464327690480265895");
}
