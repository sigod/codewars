fn generate_shape(n: i32) -> String {
	let mut result = String::with_capacity((n * (n + 1)) as usize);

	for _ in 0..n {
		for _ in 0..n {
			result.push('+');
		}

		result.push('\n');
	}

	let new_length = result.len() - 1;
	result.truncate(new_length);

	result
}

#[test]
fn sample_test() {
	assert_eq!(generate_shape(3), "+++\n+++\n+++");
}
