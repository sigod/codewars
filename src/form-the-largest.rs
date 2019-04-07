fn max_number(n: u32) -> u32 {
	let mut digits = Vec::new();
	let mut n = n;
	while n > 0 {
		digits.push(n % 10);
		n /= 10;
	}

	digits.sort();
	let mut result = 0;

	for digit in digits.into_iter().rev() {
		result = result * 10 + digit;
	}

	result
}

#[test]
fn basic_tests() {
	assert_eq!(max_number(213), 321);
	assert_eq!(max_number(7389), 9873);
	assert_eq!(max_number(63729), 97632);
	assert_eq!(max_number(566797), 977665);
	assert_eq!(max_number(17693284), 98764321);
}
