use std::iter::{self, Iterator, Sum};

struct OddTriangle {
	current: u64,
	last: u64,
}

impl OddTriangle {
	fn new(rows: u64) -> OddTriangle {
		let a1 = 1;
		let d = 2;
		let numbers_per_row = arith_an(a1, rows, 1);
		let last_index = arith_sum(a1, numbers_per_row, rows);
		let last_number = arith_an(a1, last_index, 2);
		let start_index = last_index - numbers_per_row + 1;
		let start_number = arith_an(a1, start_index, 2);

		OddTriangle {
			current: start_number,
			last: last_number,
		}
	}
}

impl Iterator for OddTriangle {
	type Item = u64;

	// because why not
	fn next(&mut self) -> Option<u64> {
		if self.current > self.last {
			None
		}
		else {
			let result = self.current;
			self.current += 2;

			Some(result)
		}
	}

	fn sum<S>(self) -> S where S: Sum<Self::Item> {
		let n = (self.last - self.current) / 2 + 1;
		let result = arith_sum(self.current, self.last, n);

		S::sum(iter::once(result))
	}
}

fn arith_sum(a1: u64, an: u64, n: u64) -> u64 {
	((n as f64 / 2.0) * (a1 + an) as f64) as u64
}

fn arith_an(a1: u64, n: u64, d: u64) -> u64 {
	a1 + (n - 1) * d
}

fn row_sum_odd_numbers(n:i64) -> i64 {
	OddTriangle::new(n as u64).sum::<u64>() as i64
}

#[test]
fn returns_expected() {
	assert_eq!(row_sum_odd_numbers(1), 1);
	assert_eq!(row_sum_odd_numbers(42), 74088);
}
