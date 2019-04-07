fn seven(n: i64) -> (i64, i32) {
	let mut steps = 0;
	let mut a = n;

	while a > 99 {
		a = (a / 10) - 2 * (a % 10);
		steps += 1;
	}

	(a, steps)
}

fn dotest(n: i64, exp: (i64, i32)) -> () {
	println!(" n: {:?};", n);
	let ans = seven(n);
	println!(" actual:\n{:?};", ans);
	println!("expect:\n{:?};", exp);
	println!(" {};", ans == exp);
	assert_eq!(ans, exp);
	println!("{};", "-");
}

#[test]
fn basic_tests() {
	dotest(477557101, (28, 7));
	dotest(477557102, (47, 7));
	dotest(1603, (7, 2));
}
