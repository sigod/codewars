
struct NumberEnd {
	value: u8,
	is_zero: bool,
}

impl NumberEnd {
	fn from_str(string: &str) -> NumberEnd {
		let mut value = last_n_chars(string, 2).parse::<u8>().unwrap();

		NumberEnd {
			value,
			is_zero: value == 0 && string.len() == 1,
		}
	}

	fn power(&self, b: &NumberEnd) -> NumberEnd {
		if b.is_zero {
			return NumberEnd { value: 1, is_zero: false };
		}

		let table: [Vec<u8>; 10] = [
			vec![0],
			vec![1],
			vec![2, 4, 8, 6],
			vec![3, 9, 7, 1],
			vec![4, 6],
			vec![5],
			vec![6],
			vec![7, 9, 3, 1],
			vec![8, 4, 2, 6],
			vec![9, 1],
		];

		let row = &table[self.value as usize % 10];

		NumberEnd {
			value: row[(b.value as usize + row.len() - 1) % row.len()],
			is_zero: false,
		}
	}

	fn last_digit(&self) -> u8 {
		self.value % 10
	}
}

fn last_n_chars(string: &str, n: usize) -> &str {
	&string[(n.max(string.len()) - n) ..]
}

fn last_digit(str1: &str, str2: &str) -> i32 {
	let a = NumberEnd::from_str(str1);
	let b = NumberEnd::from_str(str2);

	a.power(&b).last_digit() as i32
}

#[test]
fn returns_expected() {
	assert_eq!(last_digit("4", "1"), 4);
	assert_eq!(last_digit("4", "2"), 6);
	assert_eq!(last_digit("9", "7"), 9);
	assert_eq!(last_digit("10","10000000000"), 0);
	assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376","2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
	assert_eq!(last_digit("3715290469715693021198967285016729344580685479654510946723", "68819615221552997273737174557165657483427362207517952651"), 7);
	assert_eq!(last_digit("990993187088506197123393143145592118361518272976483731793299244613036593750989728883", "246117868829"), 3);
}
