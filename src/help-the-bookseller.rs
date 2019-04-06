use std::collections::HashMap;

fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
	if list_art.is_empty() {
		return "".to_string();
	}

	let mut categories = HashMap::new();

	for stock in list_art.iter() {
		let category = stock.chars().next().unwrap();

		let mut stock_iter = stock.split_whitespace();
		stock_iter.next();
		let count = stock_iter.next().unwrap().parse::<u32>().unwrap();

		if categories.contains_key(&category) {
			let map_count = categories.get_mut(&category).unwrap();
			*map_count += count;
		}
		else {
			categories.insert(category, count);
		}
	}

	let mut result = String::new();

	for category in list_cat.iter() {
		let category = category.chars().next().unwrap();
		let count = match categories.get(&category) {
			Some(count) => count,
			_ => &0,
		};

		result.push_str(&format!("({} : {}) - ", category, count));
	}

	if !result.is_empty() {
		let new_length = result.len() - " - ".len();
		result.truncate(new_length);
	}

	result
}

fn dotest(list_art: Vec<&str>, list_cat: Vec<&str>, exp: &str) -> () {
	println!("list_art: {:?};", list_art);
	println!("list_cat: {:?};", list_cat);
	let ans = stock_list(list_art, list_cat);
	println!("actual:\n{:?};", ans);
	println!("expect:\n{:?};", exp);
	println!("{};", ans == exp);
	assert_eq!(ans, exp);
	println!("{};", "-");
}

#[test]
fn basic_tests() {
	let mut b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
	let mut c = vec!["A", "B", "C", "D"];
	dotest(b, c, "(A : 0) - (B : 1290) - (C : 515) - (D : 600)");

	b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
	c = vec!["A", "B"];
	dotest(b, c, "(A : 200) - (B : 1140)");

	b = vec![];
	c = vec!["B", "R", "D", "X"];
	dotest(b, c, "");

	b = vec!["ROXANNE 102", "RHODODE 123", "BKWRKAA 125", "BTSQZFG 239", "DRTYMKH 060"];
	c = vec![];
	dotest(b, c, "");
}
