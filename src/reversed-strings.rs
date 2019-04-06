fn solution(phrase: &str) -> String {
	phrase.chars().rev().collect::<String>()
}

#[test]
fn sample_test() {
	assert_eq!(solution("world"), "dlrow");
	//assert_eq!(solution("loẅks"), "skẅol");
}
