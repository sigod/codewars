fn power_of_two(x: u64) -> bool {
	x.is_power_of_two()
}

#[test]
fn basic_tests() {
	assert_eq!(power_of_two(0), false);
	assert_eq!(power_of_two(2), true);
	assert_eq!(power_of_two(5), false);
	assert_eq!(power_of_two(6), false);
	assert_eq!(power_of_two(8), true);
	assert_eq!(power_of_two(1024), true);
	assert_eq!(power_of_two(4096), true);
	assert_eq!(power_of_two(8191), false);
}
