fn cycle(n: i64) -> i64 {
	let mut value = 10;
	let mut count = 0;

	for _ in 0..1000000 {
		if value % n == 0 {
			return -1;
		}
		if value == 10 && count > 0 {
			return count;
		}

		value = (value % n) * 10;
		count += 1;
	}

	return -1;
}

fn dotest(n: i64, exp: i64) -> () {
	let ans = cycle(n);
	assert_eq!(ans, exp)
}

#[test]
fn basic_tests() {
	dotest(33, 2);
	dotest(18118, -1);
	dotest(69, 22);
	dotest(197, 98);
	dotest(65, -1);
	dotest(3, 1);
	dotest(218713, 9744);
	dotest(437147, 20930);
}
