use std::fmt::Write;

fn dont_give_me_five(start: isize, end: isize) -> isize {
	let mut string = String::with_capacity(32);
	let mut count = 0;

	for number in start..end + 1 {
		write!(&mut string, "{}", number).unwrap();

		if string.find('5').is_none() {
			count += 1;
		}

		string.clear();
	}

	count
}

#[test]
fn returns_expected() {
	assert_eq!(dont_give_me_five(1, 9), 8);
	assert_eq!(dont_give_me_five(4, 17), 12);
}
