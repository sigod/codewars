fn is_divisible(wall: i32, pixel: i32) -> bool {
	wall % pixel == 0
}

#[test]
fn smaple_test() {
	assert_eq!(is_divisible(4050, 27), true);
	assert_eq!(is_divisible(4066, 27), false);
	assert_eq!(is_divisible(2, 4), false);
	assert_eq!(is_divisible(4, 2), true);
}
