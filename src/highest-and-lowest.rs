fn high_and_low(numbers: &str) -> String {
	let mut values = vec![std::i32::MIN, std::i32::MAX];
	numbers
		.split_whitespace()
		.map(|s| s.parse::<i32>().unwrap())
		.fold(&mut values, |acc, v| {
			acc[0] = acc[0].max(v);
			acc[1] = acc[1].min(v);

			acc
		});

	format!("{} {}", values[0], values[1])
}

#[test]
fn sample_test() {
	assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}
