// https://www.codewars.com/kata/duplicate-encoder
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

fn duplicate_encode(word: &str) -> String {
	let word = word.to_uppercase();

	let mut counts = HashMap::new();
	for c in word.chars() {
		let count = match counts.entry(c) {
			Vacant(entry) => entry.insert(0),
			Occupied(entry) => entry.into_mut(),
		};

		*count += 1;
	}

	let mut result = String::new();
	for c in word.chars() {
		if counts.get(&c).unwrap() == &1 {
			result.push('(');
		}
		else {
			result.push(')');
		}
	}

	result
}

#[test]
fn run_tests() {
	assert_eq!(duplicate_encode("din"),"(((");
	assert_eq!(duplicate_encode("recede"),"()()()");
	assert_eq!(duplicate_encode("Success"),")())())","should ignore case");
	assert_eq!(duplicate_encode("(( @"),"))((");
}
