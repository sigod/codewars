fn remove_nb(n: i32) -> Vec<(i32, i32)> {
	let n = n as i64;
	let sum = (1 + n) * n / 2;
	let mut result = Vec::new();

	// for a in 1..n-1 {
	// 	for b in a..n {
	// 		if a * b == sum - a - b {
	// 			result.push((a as i32, b as i32));
	// 		}
	// 	}
	// }
	for a in 1..=n {
		// a * b == sum - a - b
		// a * b + b == sum - a
		// (a + 1) * b == sum - a
		let b = (sum - a) / (a + 1);
		if b < n && (sum - a) % (a + 1) == 0 {
			result.push((a as i32, b as i32));
		}
	}

	// let size = result.len();
	// for i in 0..size {
	// 	let value = result[size - 1 - i];
	// 	result.push((value.1, value.0));
	// }

	result
}

fn testing(n: i32, exp: Vec<(i32, i32)>) -> () {
    assert_eq!(remove_nb(n), exp)
}

#[test]
fn basics_remove_nb() {
    testing(26, vec![(15, 21), (21, 15)]);
    testing(100, vec![]);
    testing(101, vec![(55, 91), (91, 55)]);
    testing(102, vec![(70, 73), (73, 70)]);
}
