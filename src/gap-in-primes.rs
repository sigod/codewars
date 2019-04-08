fn is_prime(number: u64) -> bool {
	if number == 2 {
		return true;
	}
	if number < 2 || number % 2 == 0 {
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

fn gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
	let g = g as u64;

	let mut prime_last = 0;

	for number in m..n {
		if is_prime(number) {
			prime_last = number;
			break;
		}
	}

	if prime_last != 0 {
		let mut number = prime_last + 2;
		while number < n {
			if is_prime(number) {
				if prime_last + g == number {
					return Some((prime_last, number));
				}
				else {
					prime_last = number;
				}
			}

			number += 2;
		}
	}

	None
}

fn testing(g: i32, m: u64, n: u64, exp: Option<(u64, u64)>) -> () {
	assert_eq!(gap(g, m, n), exp)
}

#[test]
fn basics_gap() {
	testing(2,100,110, Some((101, 103)));
	testing(4,100,110, Some((103, 107)));
	testing(6,100,110, None);
	testing(8,300,400, Some((359, 367)));
}
