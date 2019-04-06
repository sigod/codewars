fn opposite(number: i32) -> i32 {
	number * -1
}

#[test]
fn returns_expected() {
	assert_eq!(opposite(1), -1);
	assert_eq!(opposite(-1), 1);
}
