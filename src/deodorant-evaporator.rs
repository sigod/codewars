fn evaporator(content: f64, evap_per_day: i32, threshold: i32) -> i32 {
	let threshold = content * (threshold as f64 / 100.0);
	let mut day: i32 = 0;
	let mut content = content;

	loop {
		if content < threshold {
			return day;
		}

		content *= 1.0 - (evap_per_day as f64 / 100.0);
		day += 1;
	}
}

fn dotest(_content: f64, evap_per_day: i32, threshold: i32, exp: i32) -> () {
	println!(" evap_per_day: {:?}", evap_per_day);
	println!("threshold: {:?}", threshold);
	let ans = evaporator(_content, evap_per_day, threshold);
	println!(" actual:\n{:?}", ans);
	println!("expect:\n{:?}", exp);
	println!(" {};", ans == exp);
	assert_eq!(ans, exp);
	println!("{};", "-");
}

#[test]
fn basic_tests() {
	dotest(10.0, 10, 10, 22);
	dotest(10.0, 10, 5, 29);
}
