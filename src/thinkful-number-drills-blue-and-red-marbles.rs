fn guess_blue(blue_start: u32, red_start: u32, blue_pulled: u32, red_pulled: u32) -> f32 {
	let blue = blue_start - blue_pulled;
	let red = red_start - red_pulled;

	blue as f32 / (blue + red) as f32
}

#[test]
fn basic_test() {
	assert_eq!(guess_blue(5, 5, 2, 3), 0.6);
	assert_eq!(guess_blue(5, 7, 4, 3), 0.2);
	assert_eq!(guess_blue(12, 18, 4, 6), 0.4);
}
