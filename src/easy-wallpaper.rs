fn wall_paper(l: f64, w: f64, h: f64) -> String {
	if l == 0.0 || w == 0.0 || h == 0.0 {
		return "zero".to_string();
	}

	let roll_square = 0.52 * 10.0;
	let count = (1.15 * 2.0 * (l * h + w * h) / roll_square).ceil() as u32;

	match count {
		0 => "zero".to_string(),
		1 => "one".to_string(),
		2 => "two".to_string(),
		3 => "three".to_string(),
		4 => "four".to_string(),
		5 => "five".to_string(),
		6 => "six".to_string(),
		7 => "seven".to_string(),
		8 => "eight".to_string(),
		9 => "nine".to_string(),
		10 => "ten".to_string(),
		11 => "eleven".to_string(),
		12 => "twelve".to_string(),
		13 => "thirteen".to_string(),
		14 => "fourteen".to_string(),
		15 => "fifteen".to_string(),
		16 => "sixteen".to_string(),
		17 => "seventeen".to_string(),
		18 => "eighteen".to_string(),
		19 => "nineteen".to_string(),
		20 => "twenty".to_string(),
		_ => panic!(),
	}
}

fn dotest(l: f64, w: f64, h: f64, exp: &str) -> () {
	println!("l: {:?}", l);
	println!("w: {:?}", w);
	println!("h: {:?}", h);
	let ans = wall_paper(l, w, h);
	println!("actual:\n{:?}", ans);
	println!("expect:\n{:?}", exp);
	println!("{}", ans == exp);
	assert_eq!(ans, exp);
	println!("{}", "-");
}

#[test]
fn basic_tests() {
	dotest(6.3, 4.5, 3.29, "sixteen");
	dotest(6.3, 5.8, 3.13, "seventeen");
}
