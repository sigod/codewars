fn reverse_words(str: &str) -> String {
	let mut result = String::with_capacity(str.len());
	let mut word = Vec::new();
	let mut flag = true;

	let add_word = |result: &mut String, word: &mut Vec<_>| {
		for c in word.iter().rev() {
			result.push(*c);
		}

		word.clear();
	};

	for symbol in str.chars() {
		if !symbol.is_whitespace() {
			flag = true;
			word.push(symbol);
		}
		else {
			if flag {
				add_word(&mut result, &mut word);
				flag = false;
			}

			result.push(symbol);
		}
	}

	if flag {
		add_word(&mut result, &mut word);
	}

	result
}

#[test]
fn sample_test() {
	assert_eq!(reverse_words("The quick brown fox jumps over the lazy dog."), "ehT kciuq nworb xof spmuj revo eht yzal .god");
	assert_eq!(reverse_words("apple"), "elppa");
	assert_eq!(reverse_words("a b c d"),"a b c d");
	assert_eq!(reverse_words("double  spaced  words"), "elbuod  decaps  sdrow");
}
