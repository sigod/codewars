extern crate chrono;

use chrono::prelude::*;

fn date_nb_days(a0: i32, a: i32, p: i32) -> String {
	let percent = p as f64 / 100.0 / 360.0;
	let mut value = a0 as f64;
	let mut days = 0;

	while value < a as f64 {
		value = value + value * percent;
		days += 1;
	}

	Utc.ymd(2016, 01, 01)
		.checked_add_signed(chrono::Duration::days(days))
		.unwrap()
		.format("%Y-%m-%d")
		.to_string()
}

fn dotest(a0: i32, a: i32, p: i32, exp: &str) -> () {
	println!(" a0: {:?};", a0);
	println!("a: {:?};", a);
	println!("p: {:?};", p);
	let ans = date_nb_days(a0, a, p);
	println!("actual:\n{:?};", ans);
	println!("expect:\n{:?};", exp);
	println!("{};", ans == exp);
	assert_eq!(ans, exp);
	println!("{};", "-");
}

#[test]
fn basic_tests() {
	dotest(4281, 5087, 2, "2024-07-03");
	dotest(4620, 5188, 2, "2021-09-19");
}
