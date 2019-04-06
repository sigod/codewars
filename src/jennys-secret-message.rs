fn greet(input : &str) -> String {
	match input {
		"Johnny" => "Hello, my love!".to_string(),
		Any => format!("Hello, {}!", input),
	}
}

#[test]
fn greets_some_people_normally() {
	assert_eq!(greet("Jim"),   "Hello, Jim!");
	assert_eq!(greet("Jane"),  "Hello, Jane!");
	assert_eq!(greet("Simon"), "Hello, Simon!");
}

#[test]
fn greets_johnny_special() {
	assert_eq!(greet("Johnny"), "Hello, my love!");
}
