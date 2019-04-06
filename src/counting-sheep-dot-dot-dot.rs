fn count_sheep(flock: &[bool]) -> u8 {
	let mut result = 0;
	for sheep in flock {
		if *sheep == true {
			result += 1;
		}
	}

	result
}

#[test]
fn returns_correct_sheep_count() {
	assert_eq!(count_sheep(&[false]), 0);
	assert_eq!(count_sheep(&[true]), 1);
	assert_eq!(count_sheep(&[true, false]), 1);
}
