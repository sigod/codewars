fn destroy(input_sets: Vec<HashSet<char>>) -> String {
	let mut symbols: [bool; 26] = [true; 26];

	for set in input_sets.iter() {
		for c in set {
			let c = *c as i32 - 97;

			if c >= 0 && c < 26 {
				symbols[c as usize] = false;
			}
		}
	}

	let mut result = String::with_capacity(26 * 2);
	for (code, flag) in symbols.iter().enumerate() {
		if code > 0 {
			result.push(' ');
		}

		if *flag {
			result.push((code as u8 + 97) as char);
		}
		else {
			result.push('_');
		}
	}

	result
}

#[test]
fn basic_test1() {
	let mut input_set: Vec<HashSet<char>> = Vec::new();
	input_set.push(['A', 'b'].iter().cloned().collect());
	input_set.push(['C', 'd'].iter().cloned().collect());
	assert_eq!(destroy(input_set), "a _ c _ e f g h i j k l m n o p q r s t u v w x y z");
}

#[test]
fn basic_test2() {
	let mut input_set: Vec<HashSet<char>> = Vec::new();
	input_set.push(['B', 'b'].iter().cloned().collect());
	input_set.push(['C', 'm', 'f'].iter().cloned().collect());
	assert_eq!(destroy(input_set), "a _ c d e _ g h i j k l _ n o p q r s t u v w x y z");
}
