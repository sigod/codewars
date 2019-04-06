fn move_hero(position: u32, roll: u32) -> u32 {
	position + roll * 2
}

#[test]
fn sample_tests() {
	assert_eq!(move_hero(0, 4), 8);
}
