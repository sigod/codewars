fn quadratic(a: f64, b: f64, c: f64) -> f64 {
	// NOTE: https://docs.oracle.com/cd/E19957-01/806-3568/ncg_goldberg.html#5751

	let r1 = (2.0 * c) / (-b - (b*b - 4.0*a*c).sqrt());
	let r2 = (-b - (b*b - 4.0*a*c).sqrt()) / (2.0*a);

	if r1.abs() < r2.abs() {r1} else {r2}
}

fn assert_fuzzy_equals(a: f64, b: f64, c: f64) -> () {
	let merr = 1e-12;
	println!("{:?} {:e} {:?}", a, b, c);
	let x = quadratic(a, b, c);
	let smallest = x.abs() < 1.0e-1;
	if smallest == false {
		println!("This root is not the good one");
	}
	let actual = a * x * x + b * x + c;
	println!("Actual f(x) {:e}", actual);
	let inrange = actual.abs() <= merr;
	if inrange == false {
		println!("Expected value near: 0 but got: {:e}", actual);
	}
	let correct = smallest && inrange;
	assert_eq!(correct, true);
}


#[test]
fn basic_tests() {
	assert_fuzzy_equals(7.0, 4.00e+13, 8.0);
	assert_fuzzy_equals(9.0, 1.00e+14, 1.0);
	assert_fuzzy_equals(3.0, 3.00e+09, 1.0);
	assert_fuzzy_equals(7.0, 4.00e+09, 7.0);
}
