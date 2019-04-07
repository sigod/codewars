fn calc(array: Vec<i32>) -> i32 {
	let mut sum = 0;

	for (i, a) in array.into_iter().enumerate() {
		let mut a = a;

		if a > 0 {
			a *= a;
		}
		if (i + 1) % 3 == 0 {
			a *= 3;
		}
		if (i + 1) % 5 == 0 {
			a *= -1;
		}

		sum += a;
	}

	sum
}

#[test]
fn tests() {
	assert_eq!(calc(vec![0, 2, 1, -6, -3, 3]), 31);
	assert_eq!(calc(vec![0]), 0);
	assert_eq!(calc(vec![1, 1, 1, 1, 1]), 5);
	assert_eq!(calc(vec![1, 1, -9, 9, 16, -15, -45, -73, 26]), 1665);
	assert_eq!(calc(vec![1, -1, 10, -9, 16, 15, 45, -73, -26]), 2584);
	assert_eq!(calc(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
	assert_eq!(calc(vec![-5, -5, -5, -5, -5, -5, -5]), -45);
}
