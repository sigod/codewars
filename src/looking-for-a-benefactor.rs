fn new_avg(arr: &[f64], newavg: f64) -> Option<i32> {
	let required_sum = newavg * (arr.len() + 1) as f64;
	let available_sum = arr.iter().sum::<f64>();

	if available_sum > required_sum {
		None
	}
	else {
		Some((required_sum - available_sum).ceil() as i32)
	}
}

fn testing(arr: &[f64], newavg: f64, exp: Option<i32>) -> () {
	assert_eq!(exp, new_avg(arr, newavg))
}

#[test]
fn basic_tests() {
	let a1 = [14.0, 30.0, 5.0, 7.0, 9.0, 11.0, 16.0];
	testing(&a1, 90.0, Some(628));
	let a2 = [14.0, 30.0, 5.0, 7.0, 9.0, 11.0, 15.0];
	testing(&a2, 92.0, Some(645));
	let a3 = [14.0, 30.0, 5.0, 7.0, 9.0, 11.0, 15.0];
	testing(&a3, 2.0, None);
	let a4 = [14000.25, 300.76, 50.56, 70.0, 90.0, 11.0, 150.48, 1200.98];
	testing(&a4, 4800.0, Some(27326));
}
