use std::collections::HashMap;

const STATES: [(&str, &str); 8] = [
	("AZ", "Arizona"),
	("CA", "California"),
	("ID", "Idaho"),
	("IN", "Indiana"),
	("MA", "Massachusetts"),
	("OK", "Oklahoma"),
	("PA", "Pennsylvania"),
	("VA", "Virginia"),
];

fn by_state(str: &str) -> String {
	let mut found_states = HashMap::new();

	for address in str.split("\n") {
		let state_short = &address[address.len() - 2..];

		found_states
			.entry(state_short)
			.or_insert(vec![])
			.push(&address[..address.len() - 3]);
	}

	let mut result = String::new();

	for state in STATES.iter() {
		if let Some(mut addresses) = found_states.get_mut(state.0) {
			addresses.sort();

			if result.len() > 0 {
				result.push(' ');
			}
			result.push_str(&format!("{}\n", state.1));

			for address in addresses {
				result.push_str(&format!("..... {} {}\n", address.replace(",", ""), state.1));
			}
		}
	}

	let new_length = result.len() - "\n".len();
	result.truncate(new_length);

	result
}

fn dotest(s: &str, exp: &str) -> () {
	println!("s:{}", s);
	let ans = by_state(s);
	println!("actual:\n{}", ans);
	println!("expect:\n{}", exp);
	println!("{}", ans == exp);
	assert_eq!(ans, exp);
	println!("{}", "-");
}

#[test]
fn basic_tests() {
	let ad="John Pulsett, 321 King Street, Palmouth MA\nAlisa Gord, 22 Prin Broadway, Georges VA\nOreste Thulas, 11354 East Bridge Road, Pensa OK\nPerry Falpas, 420 Land Road, Beaver Halls PA\nErica Adamson, 200 Station Road, Westbury MA\nPaulo Sims, 8A River Street, Richmond VA\nAnn Wildon, 334 Shore Parkway, Hill View CA\nAl Carpenter, 730 3rd Street, Boston MA";
	let adsol = "California\n..... Ann Wildon 334 Shore Parkway Hill View California\n Massachusetts\n..... Al Carpenter 730 3rd Street Boston Massachusetts\n..... Erica Adamson 200 Station Road Westbury Massachusetts\n..... John Pulsett 321 King Street Palmouth Massachusetts\n Oklahoma\n..... Oreste Thulas 11354 East Bridge Road Pensa Oklahoma\n Pennsylvania\n..... Perry Falpas 420 Land Road Beaver Halls Pennsylvania\n Virginia\n..... Alisa Gord 22 Prin Broadway Georges Virginia\n..... Paulo Sims 8A River Street Richmond Virginia";
	dotest(ad, adsol);

	let ad3="John Daggett, 341 King Road, Plymouth MA\nAlice Ford, 22 East Broadway, Richmond VA\nSal Carpenter, 73 6th Street, Boston MA";
	let ad3sol="Massachusetts\n..... John Daggett 341 King Road Plymouth Massachusetts\n..... Sal Carpenter 73 6th Street Boston Massachusetts\n Virginia\n..... Alice Ford 22 East Broadway Richmond Virginia";
	dotest(ad3, ad3sol);
}
