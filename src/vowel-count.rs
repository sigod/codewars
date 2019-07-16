fn get_count(string: &str) -> usize {
	string.chars()
		.filter(|&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u')
		.count()
}

#[test]
fn my_tests() {
	assert_eq!(get_count("abracadabra"), 5);
}
