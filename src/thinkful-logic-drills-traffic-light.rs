fn update_light(current: &str) -> String {
	match current {
		"green" => "yellow",
		"yellow" => "red",
		"red" => "green",
		_ => panic!("Unknown color '{}'!", current),
	}.to_string()
}

#[test]
fn basic_test() {
	assert_eq!(update_light("green"), "yellow");
	assert_eq!(update_light("yellow"), "red");
	assert_eq!(update_light("red"), "green");
}
