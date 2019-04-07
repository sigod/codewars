fn product_fib(product: u64) -> (u64, u64, bool) {
	let mut sequence = vec![0, 1, 1];

	for i in 1.. {
		if sequence[i - 1] * sequence[i] == product {
			return (sequence[i - 1], sequence[i], true);
		}
		else if product < sequence[i] * sequence[i + 1] {
			return (sequence[i], sequence[i + 1], false);
		}

		let next_number = sequence[sequence.len() - 2] + sequence[sequence.len() - 1];
		sequence.push(next_number);
	}

	panic!();
}

fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
	assert_eq!(product_fib(prod), exp)
}

#[test]
fn basics_product_fib() {
	dotest(4895, (55, 89, true));
	dotest(5895, (89, 144, false));
}
