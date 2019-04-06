fn xo(string: &'static str) -> bool {
	let mut x = 0;
	let mut o = 0;

	for c in string.chars() {
		match c {
			'x' => x += 1,
			'X' => x += 1,
			'o' => o += 1,
			'O' => o += 1,
			_ => (),
		}
	}

	x == o
}

#[test]
fn returns_expected() {
	assert_eq!(xo("xo"), true);
	assert_eq!(xo("Xo"), true);
	assert_eq!(xo("xxOo"), true);
	assert_eq!(xo("xxxm"), false);
	assert_eq!(xo("Oo"), false);
	assert_eq!(xo("ooom"), false);
}
