fn two_sort(arr: &[&str]) -> String {
	let mut strings = Vec::from(arr);
	strings.sort_unstable();

	let mut result = String::new();
	let mut word = strings[0].chars();

	result.push(word.next().unwrap());
	for c in word {
		result.push_str("***");
		result.push(c);
	}

	result
}

#[test]
fn sample_test_cases() {
	assert_eq!(two_sort(&["bitcoin", "take", "over", "the", "world", "maybe", "who", "knows", "perhaps"]), "b***i***t***c***o***i***n");
	assert_eq!(two_sort(&["turns", "out", "random", "test", "cases", "are", "easier", "than", "writing", "out", "basic", "ones"]), "a***r***e");
}
