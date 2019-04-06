fn repeat_str(src: &str, count: usize) -> String {
	let mut result = String::with_capacity(src.len() * count);

	for _ in 0..count {
		result.push_str(src);
	}

	result
}

#[test]
fn example_tests() {
	assert_eq!(repeat_str("a", 4), "aaaa");
	assert_eq!(repeat_str("hello ", 3), "hello hello hello ");
	assert_eq!(repeat_str("abc", 2), "abcabc");
}
