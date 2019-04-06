fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
	if input.len() == 0 {
		return vec![];
	}

	let mut result = vec![0, 0];

	for number in input {
		if number > 0 {
			result[0] += 1;
		}
		else {
			result[1] += number;
		}
	}

	result
}

#[test]
fn returns_expected() {
	let test_data1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15];
	let expected1 = vec![10, -65];
	assert_eq!(count_positives_sum_negatives(test_data1), expected1);
}
