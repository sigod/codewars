fn nth_even(n: u32) -> u32 {
	match n {
		1 => 0,
		n => (n - 1) * 2,
	}
}

#[test]
fn example_tests() {
	assert_eq!(nth_even(1), 0);
	assert_eq!(nth_even(2), 2);
	assert_eq!(nth_even(3), 4);
	assert_eq!(nth_even(100), 198);
	assert_eq!(nth_even(1298734), 2597466);
}
