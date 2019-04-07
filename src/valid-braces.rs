fn valid_braces(s: &str) -> bool {
	let mut stack = vec![];

	for c in s.chars() {
		match c {
			'(' => stack.push(')'),
			'[' => stack.push(']'),
			'{' => stack.push('}'),
			c => {
				if Some(c) != stack.pop() {
					return false;
				}
			},
		}
	}

	stack.is_empty()
}

#[test]
fn basic_tests() {
	expect_true("()");
	expect_false("[(])");
}
