fn divisors(integer: u32) -> Result<Vec<u32>, String> {
	let mut result = Vec::new();

	for i in 2 .. integer {
		if integer % i == 0 {
			result.push(i);
		}
	}

	if result.len() > 0 {
		Ok(result)
	}
	else {
		Err(String::from(format!("{} is prime", integer)))
	}
}

#[test]
fn tests() {
	assert_eq!(divisors(15), Ok(vec![3,5]));
	assert_eq!(divisors(12), Ok(vec![2,3,4,6]));
	assert_eq!(divisors(13), Err("13 is prime".to_string()));
}
