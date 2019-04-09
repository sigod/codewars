fn interpreter(tape: &str, data: &str) -> String {
	let mut result = data.chars().collect::<Vec<char>>();
	let mut data_index = 0;
	let mut tape_index = 0;

	while data_index < result.len() {
		match tape.chars().nth(tape_index).unwrap() {
			'1' => {
				let next_bit = match result[data_index] {
					'0' => '1',
					'1' => '0',
					_ => panic!(),
				};

				result[data_index] = next_bit;
			},
			'0' => {
				data_index += 1;
			},
			_ => panic!(),
		};

		tape_index += 1;
		if tape_index == tape.len() {
			tape_index = 0;
		}
	}

	result.iter().collect()
}

#[test]
fn basic_tests() {
	// flip every bit
	assert_eq!(interpreter("10", "1010101"), "0101010");
	// flip every other bit
	assert_eq!(interpreter("100", "1111111111"), "0101010101");
}
