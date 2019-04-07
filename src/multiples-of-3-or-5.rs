fn solution(num: i32) -> i32 {
	let mut result = 0;

	for n in 1..num {
		if n % 3 == 0 || n % 5 == 0 {
			result += n;
		}
	}

	result
}

#[test]
fn returns_expected() {
	assert_eq!(solution(10), 23);
	assert_eq!(solution(11), 33);
	assert_eq!(solution(6), 8);
}
