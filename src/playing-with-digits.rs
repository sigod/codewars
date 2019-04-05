fn dig_pow(n: i64, p: i32) -> i64 {
	let p = p as u32;

	let digits = n.to_string().chars().map(|c| c.to_digit(10).unwrap() as i64).collect::<Vec<_>>();
	let nk = digits.iter().enumerate().fold(0, |nk, (i, &digit)| nk + digit.pow(p + i as u32));

	if nk % n == 0 {
		nk / n
	}
	else {
		-1
	}
}

fn dotest(n: i64, p: i32, exp: i64) -> () {
	println!(" n: {:?};", n);
	println!("p: {:?};", p);
	let ans = dig_pow(n, p);
	println!(" actual:\n{:?};", ans);
	println!("expect:\n{:?};", exp);
	println!(" {};", ans == exp);
	assert_eq!(ans, exp);
	println!("{};", "-");
}

#[test]
fn basic_tests() {
	dotest(89, 1, 1);
	dotest(92, 1, -1);
	dotest(46288, 3, 51);
}
