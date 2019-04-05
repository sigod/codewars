fn row_weights(array: Vec<u32>) -> (u32, u32) {
	let mut a = 0;
	let mut b = 0;

	for (i, weight) in array.iter().enumerate() {
		if (i + 1) % 2 == 1 {
			a += weight;
		}
		else {
			b += weight;
		}
	}

	(a, b)
}

#[test]
fn basic_tests() {
	assert_eq!(row_weights(vec![13, 27, 49]), (62, 27));
	assert_eq!(row_weights(vec![50, 60, 70, 80]), (120, 140));
	assert_eq!(row_weights(vec![80]), (80,0));
}
