fn reverse_seq(n: u32) -> Vec<u32> {
	let mut result = Vec::with_capacity(n as usize);
	for value in 0..n {
		result.push(n - value);
	}

	result
}

#[test]
fn sample_test() {
	assert_eq!(reverse_seq(5), [5, 4, 3, 2, 1].to_vec());
}
