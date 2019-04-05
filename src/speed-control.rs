fn gps(s: i32, x: Vec<f64>) -> i32 {
	let mut max = 0.0;

	for i in 1..x.len() {
		let delta = x[i] - x[i - 1];
		let speed = (3600.0 * delta) / s as f64;

		if speed > max {
			max = speed;
		}
	}

	max.trunc() as i32
}

fn dotest(s: i32, x: Vec<f64>, exp: i32) -> () {
	println!("s: {:?};", s);
	println!("x: {:?};", x);
	let ans = gps(s, x);
	println!("actual:\n{:?};", ans);
	println!("expect:\n{:?};", exp);
	println!("{};", ans == exp);
	assert_eq!(ans, exp);
	println!("{};", "-");
}

#[test]
fn basic_tests() {
	let mut x = vec![0.0, 0.23, 0.46, 0.69, 0.92, 1.15, 1.38, 1.61];
	let mut s = 20;
	let mut u = 41;
	dotest(s, x, u);
	x = vec![0.0, 0.11, 0.22, 0.33, 0.44, 0.65, 1.08, 1.26, 1.68, 1.89, 2.1, 2.31, 2.52, 3.25];
	s = 12;
	u = 219;
	dotest(s, x, u);
}
