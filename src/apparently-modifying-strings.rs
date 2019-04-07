fn apparently(string: &str) -> String {
	let mut result = String::with_capacity(string.len());
	let words = string.split_whitespace().collect::<Vec<_>>();

	for i in 0..words.len() {
		if i != 0 {
			result.push(' ');
		}

		result.push_str(words[i]);

		if (words[i] == "but" || words[i] == "and")
			&& (i == words.len() - 1 || words[i + 1] != "apparently")
		{
			result.push_str(" apparently");
		}
	}

	result
}

fn test_exp(a: &str, exp: &str) {
	assert_eq!(apparently(a), exp.to_string());
}

#[test]
fn test_apparently() {
	test_exp("It was great and I have never been on live television before but sometimes I dont watch this.", "It was great and apparently I have never been on live television before but apparently sometimes I dont watch this.");
	test_exp("and", "and apparently");
	test_exp("apparently", "apparently");
}
