fn part_list(arr: Vec<&str>) -> String {
	let mut result = String::new();

	for part_count in 1 .. arr.len() {
		result.push('(');

		result.push_str(arr[0]);
		for i in 1 .. part_count {
			result.push(' ');
			result.push_str(arr[i]);
		}
		result.push(',');
		for i in part_count .. arr.len() {
			result.push(' ');
			result.push_str(arr[i]);
		}

		result.push(')');
	}

	result
}

fn dotest(arr: Vec<&str>, exp: &str) -> () {
	println!("arr: {:?}", arr);
	let ans = part_list(arr);
	println!("actual:\n{}", ans);
	println!("expect:\n{}", exp);
	println!("{}", ans == exp);
	assert_eq!(ans, exp);
	println!("{}", "-");
}

#[test]
fn basis_tests() {
	dotest(vec!["I", "wish", "I", "hadn't", "come"],
			"(I, wish I hadn't come)(I wish, I hadn't come)(I wish I, hadn't come)(I wish I hadn't, come)");
	dotest(vec!["cdIw", "tzIy", "xDu", "rThG"],
		"(cdIw, tzIy xDu rThG)(cdIw tzIy, xDu rThG)(cdIw tzIy xDu, rThG)");
}
