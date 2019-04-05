fn camel_case(string: &str) -> String {
	let mut result = String::with_capacity(string.len());

	for word in string.split_whitespace() {
		if word.is_empty() {
			continue;
		}

		word.chars().nth(0).unwrap().to_uppercase().for_each(|c| result.push(c));
		result.push_str(&word[1..]);
	}

	result
}

#[test]
fn sample_test() {
	assert_eq!(camel_case("test case"), "TestCase");
	assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
	assert_eq!(camel_case("say hello "), "SayHello");
	assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
	assert_eq!(camel_case(""), "");
}
