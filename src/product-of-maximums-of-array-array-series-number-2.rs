fn max_product(mut lst: Vec<u32>, n_largest_elements: i32) -> u32 {
	lst.sort();
	lst.reverse();

	let mut result = 1;

	for i in 0..n_largest_elements {
		result *= lst[i as usize];
	}

	result
}

#[test]
fn basic_tests() {
	assert_eq!(max_product(vec![0; 10], 5), 0);
	assert_eq!(max_product(vec![4, 3, 5], 2), 20);
	assert_eq!(max_product(vec![10, 8, 7, 9], 3), 720);
	assert_eq!(max_product(vec![8, 6, 4, 6], 3), 288);
}
