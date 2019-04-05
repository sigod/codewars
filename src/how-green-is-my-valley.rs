fn make_valley(array: Vec<i32>) -> Vec<i32> {
	let mut array = array.to_vec();
	array.sort_unstable_by(|a, b| b.cmp(a));

	let mut result = vec![0; array.len()];
	let length = result.len();

	for i in 0..array.len() / 2 {
		result[i] = array[i * 2];
		result[length - 1 - i] = array[i * 2 + 1];
	}

	if length % 2 == 1 {
		result[length / 2] = array[length - 1];
	}

	result
}

fn dotest(arr: Vec<i32>, exp: Vec<i32>) -> () {
	println!("arr: {:?}", arr);
	let ans = make_valley(arr);
	println!("actual:\n{:?}", ans);
	println!("expect:\n{:?}", exp);
	println!("{}", ans == exp);
	assert_eq!(ans, exp);
	println!("{}", "-");
}

#[test]
fn basic_tests() {
	dotest(vec![17, 17, 15, 14, 8, 7, 7, 5, 4, 4, 1], vec![17, 15, 8, 7, 4, 1, 4, 5, 7, 14, 17]);
	dotest(vec![20, 7, 6, 2], vec![20, 6, 2, 7]);
}
