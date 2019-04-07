fn summy(string: &str) -> i32 {
	string.split_whitespace().fold(0, |acc, substring| acc + substring.parse::<i32>().unwrap())
}

#[test]
fn sample_tests() {
	assert_eq!(summy("1 2 3"), 6);
	assert_eq!(summy("1 2 3 4"), 10);
	assert_eq!(summy("1 2 3 4 5"), 15);
	assert_eq!(summy("10 10"), 20);
	assert_eq!(summy("0 0"), 0);
}
