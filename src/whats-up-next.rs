fn next_item<T: PartialEq<T> + Clone>(slice: &[T], find: T) -> Option<T> {
	let mut i = 0;
	for v in slice {
		if v == &find {
			if i + 1 < slice.len() {
				return Some(slice[i + 1].clone());
			}

			break;
		}

		i += 1;
	}

	return None;
}

#[test]
fn returns_expected() {
	assert_eq!(next_item(&["Joe", "Bob", "Sally"], "Bob"), Some("Sally"));
	assert_eq!(next_item(&[0, 1], 0), Some(1));
	assert_eq!(next_item(&[0, 1], 1), None);
	assert_eq!(next_item((1..10).collect::<Vec<_>>().as_slice(), 7), Some(8));
}
