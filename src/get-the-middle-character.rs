fn get_middle(s:&str) -> &str {
	let half = s.len() / 2;
	match s.len() % 2 {
		0 => &s[half - 1 .. half + 1],
		1 => &s[half .. half + 1],
		_ => panic!(),
	}
}

#[test]
fn example_tests() {
	assert_eq!(get_middle("test"),"es");
	assert_eq!(get_middle("testing"),"t");
	assert_eq!(get_middle("middle"),"dd");
	assert_eq!(get_middle("A"),"A");
	assert_eq!(get_middle("of"),"of");
}
