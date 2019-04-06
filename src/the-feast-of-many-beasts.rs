fn feast(beast: &str, dish: &str) -> bool {
	let mut beast_iter = beast.chars();
	let mut dish_iter = dish.chars();

	beast_iter.next().unwrap() == dish_iter.next().unwrap()
		&& beast_iter.next_back().unwrap() == dish_iter.next_back().unwrap()
}

#[test]
fn sample_test_cases() {
	assert_eq!(feast("great blue heron", "garlic naan"), true);
	assert_eq!(feast("chickadee", "chocolate cake"), true);
	assert_eq!(feast("brown bear", "bear claw"), false);
}
