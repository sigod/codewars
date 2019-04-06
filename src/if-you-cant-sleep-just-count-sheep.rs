fn countSheep(n: u32) -> String {
	let mut result = String::with_capacity("x sheep...".len().checked_mul(n as usize).unwrap());

	for i in 0..n {
		result.push_str(&format!("{} sheep...", i + 1));
	}

	result
}

#[test]
fn returns_expected() {
	assert_eq!(countSheep(1), "1 sheep...");
	assert_eq!(countSheep(2), "1 sheep...2 sheep...");
	assert_eq!(countSheep(3), "1 sheep...2 sheep...3 sheep...");
}
