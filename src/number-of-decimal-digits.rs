fn digits(n: u64) -> usize {
	let mut result: usize = 0;
	let mut n = n;
	loop {
		result += 1;
		n /= 10;

		if n == 0 {
			return result;
		}
	}
}

#[test]
fn sample_test() {
	assert_eq!(digits(5), 1);
	assert_eq!(digits(12345), 5);
	assert_eq!(digits(9876543210), 10);
}
