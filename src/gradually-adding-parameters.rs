fn add(args: &[i64]) -> i64 {
	args.into_iter().enumerate().map(|(i, a)| *a * (i + 1) as i64).sum()
}

#[test]
fn basic_tests() {
	assert_eq!(add(&[]), 0);
	assert_eq!(add(&[4,-3,-2]), -8);
}
