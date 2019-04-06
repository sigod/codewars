fn sum_of_n(n: i32) -> Vec<i32> {
	let sign = match n {
		_ if n < 0 => -1,
		_ => 1,
	};

	let n = n.abs();

	let mut result = Vec::with_capacity(n as usize + 1);

	result.push(0);
	let mut last = 0;

	for i in 1..=n {
		last += i;
		result.push(sign * last);
	}

	result
}

#[test]
fn normal_tests() {
	assert_eq!(sum_of_n(3), vec![0, 1, 3, 6]);
	assert_eq!(sum_of_n(-4), vec![0, -1, -3, -6, -10]);
	assert_eq!(sum_of_n(1), vec![0, 1]);
	assert_eq!(sum_of_n(0), vec![0]);
	assert_eq!(sum_of_n(10), vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
}
