fn is_prime(number: u64) -> bool {
	if number % 2 == 0 {
		return false;
	}

	let mut divisor = 3;
	while (divisor as f64) < (number as f64).sqrt() + 1.0 {
		if number % divisor == 0 {
			return false;
		}

		divisor += 2;
	}

	true
}

fn step(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
	let g = g as u64;

	if g % 2 == 1 || g > n {
		return None;
	}

	let mut number = if m % 2 == 0 {m + 1} else {m};
	while number <= n - g {
		if is_prime(number) && is_prime(number + g) {
			return Some((number, number + g));
		}

		number += 2;
	}

	None
}

fn testing(g: i32, m: u64, n: u64, exp: Option<(u64, u64)>) -> () {
	assert_eq!(step(g, m, n), exp)
}

#[test]
fn basics_step() {
	testing(2,100,110, Some((101, 103)));
	testing(4,100,110, Some((103, 107)));
	testing(8,30000,100000, Some((30089, 30097)));
	testing(11,30000,100000, None);
	testing(2,10000000,11000000, Some((10000139, 10000141)));
	testing(100, 20, 30, None);
	testing(8, 300, 400, Some((359, 367)));
}
