fn nb_dig(n: i32, d: i32) -> i32 {
	let mut count = 0;

	if d == 0 {
		count += 1;
	}

	for k in 1..=n {
		let mut k2 = k.pow(2);

		while k2 > 0 {
			if k2 % 10 == d {
				count += 1;
			}

			k2 /= 10;
		}
	}

	count
}

fn dotest(n: i32, d: i32, exp: i32) -> () {
	println!("n: {:?}", n);
	println!("d: {:?}", d);
	let ans = nb_dig(n, d);
	println!("actual:\n{:?}", ans);
	println!("expect:\n{:?}", exp);
	println!("{}", ans == exp);
	assert_eq!(ans, exp);
	println!("{}", "-");
}

#[test]
fn basic_tests() {
	dotest(550, 5, 213);
	dotest(5750, 0, 4700);
}
