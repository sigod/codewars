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

fn sum_of_divided(array: Vec<i64>) -> Vec<(i64, i64)> {
	if array.is_empty() {
		return vec![];
	}

	let max_value = array.iter().map(|value| value.abs()).max().unwrap();

	let mut results = Vec::new();

	for prime in 2..max_value + 1 {
		if !is_prime(prime as u64) {
			continue;
		}

		let mut is_match_found = false;
		let mut sum = 0;

		for value in array.iter() {
			if value % prime == 0 {
				is_match_found = true;
				sum += value;
			}
		}

		if is_match_found {
			results.push((prime, sum));
		}
	}

	results
}

fn testing(l: Vec<i64>, exp: Vec<(i64, i64)>) -> () {
	assert_eq!(sum_of_divided(l), exp)
}

#[test]
fn basics_sum_of_divided() {
	testing(vec![12, 15], vec![ (2, 12), (3, 27), (5, 15) ]);
	testing(vec![15,21,24,30,45], vec![ (2, 54), (3, 135), (5, 90), (7, 21) ]);
	testing(vec![107, 158, 204, 100, 118, 123, 126, 110, 116, 100],
		vec![(2,1032), (3,453), (5,310), (7,126), (11,110), (17,204), (29,116), (41,123), (59,118), (79,158), (107,107)]);
	testing(vec![], vec![]);
	testing(vec![37, 13, 19, 33, -18, 79, 26], vec![(2, 8), (3, 15), (11, 33), (13, 39), (19, 19), (37, 37), (79, 79)]);
}
