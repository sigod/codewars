fn nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {
	let percent = 1.0 + percent / 100.0;

	let mut current = p0;
	let mut year = 0;

	while current < p {
		current = (current as f64 * percent + aug as f64) as i32;
		year += 1;
	}

	year
}

fn dotest(p0: i32, percent: f64, aug: i32, p: i32, exp: i32) -> () {
	println!("p0: {:?};", p0);
	println!("percent: {:?};", percent);
	println!("aug: {:?};", aug);
	println!("p: {:?};", p);
	let ans =nb_year(p0, percent, aug, p);
	println!("actual:\n{:?};", ans);
	println!("expect:\n{:?};", exp);
	println!("{};", ans == exp);
	assert_eq!(ans, exp);
	println!("{};", "-");
}

#[test]
fn basic_tests() {
	dotest(1500, 5.0, 100, 5000, 15);
	dotest(1500000, 2.5, 10000, 2000000, 10);
}
