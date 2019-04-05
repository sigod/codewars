fn contain_all_rots(string: &str, array: Vec<&str>) -> bool {
	if string == "" {
		return true;
	}

	let mut rotations = {
		let chars = string.chars().collect::<Vec<_>>();
		let mut rotations = Vec::with_capacity(chars.len());

		for split_index in 0..chars.len() {
			let mut rotation = String::with_capacity(chars.len());

			for i in split_index..chars.len() {
				rotation.push(chars[i]);
			}
			for i in 0..split_index {
				rotation.push(chars[i]);
			}

			rotations.push(rotation);
		}

		rotations
	};

	rotations.sort();
	rotations.dedup();

	let mut array = array.to_vec();
	array.sort();
	array.dedup();

	let mut i = 0;
	let mut j = 0;

	while i < rotations.len() && j < array.len() {
		if rotations[i] == array[j] {
			i += 1;
			j += 1;
		}
		else if &*rotations[i] > array[j] {
			j += 1;
		}
		else if &*rotations[i] < array[j] {
			break;
		}
	}

	i == rotations.len()
}

fn dotest(strng: &str, arr: Vec<&str>, exp: bool) -> () {
	println!("strng: {}", strng);
	println!("arr: {:?}", arr);
	let ans = contain_all_rots(strng, arr);
	println!("actual:\n{}", ans);
	println!("expect:\n{}", exp);
	println!("{}", ans == exp);
	assert_eq!(ans, exp);
	println!("{}", "-");
}

#[test]
fn basis_tests() {
	dotest("", vec![], true);
	dotest("ab", vec!["ab", "ba"], true);
	dotest("bsjq", vec!["bsjq", "qbsj", "sjqb", "twZNsslC", "jqbs"], true);
	dotest("XjYABhR", vec!["TzYxlgfnhf", "yqVAuoLjMLy", "BhRXjYA", "YABhRXj", "hRXjYAB", "jYABhRX", "XjYABhR", "ABhRXjY"], false);
	dotest("12341234", vec!["DIeF", "IeFD", "12341234", "41234123", "34123412", "23412341"], true);
}
