fn prime_factors(n: i64) -> String {
	assert!(n > 1);

	let mut result = String::new();
	let mut value = n;

	for divider in 2..n + 1 {
		let mut count = 0;

		while value % divider == 0 {
			value /= divider;
			count += 1;
		}

		if count == 1 {
			result.push_str(&format!("({})", divider));
		}
		else if count > 1 {
			result.push_str(&format!("({}**{})", divider, count));
		}

		if value == 1 {
			break;
		}
	}

	result
}

fn testing(n: i64, exp: &str) -> () {
	assert_eq!(&prime_factors(n), exp)
}

#[test]
fn basics_prime_factors() {
	testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
	testing(17*17*93*677, "(3)(17**2)(31)(677)");
}
