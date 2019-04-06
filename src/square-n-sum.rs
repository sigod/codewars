fn square_sum(vec: Vec<i32>) -> i32 {
	let mut result = 0;
	for number in vec {
		result += number * number;
	}

	result
}

#[test]
fn returns_expected() {
	assert_eq!(square_sum(vec![1, 2]), 5);
	assert_eq!(square_sum(vec![-1, -2]), 5);
	assert_eq!(square_sum(vec![5, 3, 4]), 50);
}
