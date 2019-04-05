use std::collections::HashSet;

fn remove_duplicate_words(s: &str) -> String {
	let mut result = String::new();

	let mut unique_words = HashSet::new();
	for word in s.split_whitespace() {
		if !unique_words.contains(word) {
			unique_words.insert(word);

			result.push_str(word);
			result.push(' ');
		}
	}

	let new_length = result.len() - 1;
	result.truncate(new_length);

	result
}

#[test]
fn sample_test_cases() {
	assert_eq!(remove_duplicate_words("alpha beta beta gamma gamma gamma delta alpha beta beta gamma gamma gamma delta"), "alpha beta gamma delta");
	assert_eq!(remove_duplicate_words("my cat is my cat fat"), "my cat is fat");
}
