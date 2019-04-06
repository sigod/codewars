use std::iter::FromIterator;

fn longest(a1: &str, a2: &str) -> String {
	let mut chars = a1.chars().collect::<Vec<char>>();

	for c in a2.chars() {
		chars.push(c);
	}

	chars.sort();
	chars.dedup();

	String::from_iter(chars)
}

fn testing(s1: &str, s2: &str, exp: &str) -> () {
	println!("s1:{:?} s2:{:?}", s1, s2);
	println!("{:?} {:?}", longest(s1, s2), exp);
	println!("{}", longest(s1, s2) == exp);
	assert_eq!(&longest(s1, s2), exp)
}

#[test]
fn basic_tests() {
	testing("aretheyhere", "yestheyarehere", "aehrsty");
	testing("loopingisfunbutdangerous", "lessdangerousthancoding", "abcdefghilnoprstu");
}
