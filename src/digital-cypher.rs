fn encode(msg: String, n: i32) -> Vec<i32> {
	let code = n.to_string().chars().map(|c| c as i32 - '0' as i32).collect::<Vec<_>>();

	msg.chars()
		.enumerate()
		.map(|(i, c)| {
			let code_index = i % code.len();

			(c as i32 - 'a' as i32 + 1) + code[code_index]
		})
		.collect()
}

#[test]
fn fixed_tests() {
	assert_eq!(encode("scout".to_string(), 1939), vec![20, 12, 18, 30, 21]);
	assert_eq!(encode("masterpiece".to_string(), 1939), vec![14, 10, 22, 29, 6, 27, 19, 18, 6, 12, 8]);
}
