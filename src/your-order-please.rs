use std::collections::HashMap;

fn order(sentence: &str) -> String {
	const MAX_COUNT: usize = 9;
	let mut map = HashMap::with_capacity(MAX_COUNT);

	for word in sentence.split_whitespace() {
		let order = word.chars().filter(char::is_ascii_digit).nth(0).unwrap().to_digit(10).unwrap() as usize;

		map.insert(order, word);
	}

	let mut result = String::with_capacity(sentence.len());

	for i in 1..MAX_COUNT+1 {
		match map.get(&i) {
			Some(word) => {
				result.push_str(word);
				result.push(' ');
			},
			None => break,
		};
	}

	result.truncate(sentence.len());

	result
}

#[test]
fn returns_expected() {
	assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
	assert_eq!(order(""), "");
}
