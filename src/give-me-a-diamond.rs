use std::iter;

fn print(n: i32) -> Option<String> {
	if n < 0 || n % 2 == 0 {
		return None;
	}

	let mut diamond = String::new();
	let mut i = 1;

	while i < n {
		iter::repeat(' ').take(((n - i) / 2) as usize).for_each(|c| diamond.push(c));
		iter::repeat('*').take(i as usize).for_each(|c| diamond.push(c));
		diamond.push('\n');
		i += 2;
	}
	while i > 0 {
		iter::repeat(' ').take(((n - i) / 2) as usize).for_each(|c| diamond.push(c));
		iter::repeat('*').take(i as usize).for_each(|c| diamond.push(c));
		diamond.push('\n');
		i -= 2;
	}

	Some(diamond)
}

#[test]
fn basic_test() {
	assert_eq!(print(3), Some(" *\n***\n *\n".to_string()) );
	assert_eq!(print(-3),None);
	assert_eq!(print(1), Some("*\n".to_string()) );
}
