fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
	if a1.is_empty() || a2.is_empty() {
		return -1;
	}

	let a1 = a1.iter().map(|s| s.len() as i32);
	let a2 = a2.iter().map(|s| s.len() as i32);

	(a1.clone().max().unwrap() - a2.clone().min().unwrap())
		.max(a2.max().unwrap() - a1.min().unwrap())
}

fn dotest(a1: Vec<&str>, a2: Vec<&str>, exp: i32) -> () {
	println!("a1: {:?};", a1);
	println!("a2: {:?};", a2);
	let ans = mx_dif_lg(a1, a2);
	println!("actual:\n{:?};", ans);
	println!("expect:\n{:?};", exp);
	println!("{};", ans == exp);
	assert_eq!(ans, exp);
	println!("{};", "-");
}

#[test]
fn basic_tests() {
	let mut s1 = vec!["hoqq", "bbllkw", "oox", "ejjuyyy", "plmiis", "xxxzgpsssa", "xxwwkktt", "znnnnfqknaz", "qqquuhii", "dvvvwz"];
	let mut s2 = vec!["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"];
	dotest(s1, s2, 13);
	s1 = vec!["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", "oocccffuucccjjjkkkjyyyeehh"];
	s2 = vec!["bbbaaayddqbbrrrv"];
	dotest(s1, s2, 10);
}
