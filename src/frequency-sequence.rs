use std::collections::HashMap;

fn freq_seq(s: &str, sep: &str) -> String {
	let mut chars = HashMap::new();

	for c in s.chars() {
		if chars.contains_key(&c) {
			if let Some(value) = chars.get_mut(&c) {
				*value += 1;
			}
			else {
				panic!();
			}
		}
		else {
			chars.insert(c, 1);
		}
	}

	let mut result = String::with_capacity((1 + sep.len()) * s.len());

	for c in s.chars() {
		result.push_str(&chars[&c].to_string());
		result.push_str(sep);
	}

	let new_length = result.len() - sep.len();
	result.truncate(new_length);

	result
}

#[test]
fn returns_expected() {
	assert_eq!(freq_seq("hello world", "-"), "1-1-3-3-2-1-1-2-1-3-1");
	assert_eq!(freq_seq("19999999", ":"), "1:7:7:7:7:7:7:7");
	assert_eq!(freq_seq("^^^**$", "x"), "3x3x3x2x2x1");
}
