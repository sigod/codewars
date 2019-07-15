fn positive_sum(array: &[i32]) -> i32 {
	let mut result = 0;

	for &value in array.iter() {
		if value > 0 {
			result += value;
		}
	}

	result
}

#[test]
fn some_examples() {
	assert_eq!(positive_sum(&[1,2,3,4,5]), 15);
	assert_eq!(positive_sum(&[1,-2,3,4,5]), 13);
	assert_eq!(positive_sum(&[-1,2,3,4,-5]), 9);
}

#[test]
fn empty_list() {
	assert_eq!(positive_sum(&[]), 0);
}

#[test]
fn all_negative() {
	assert_eq!(positive_sum(&[-1,-2,-3,-4,-5]), 0);
}
