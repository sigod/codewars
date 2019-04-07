fn good_vs_evil(good: &str, evil: &str) -> String {
	const RACES_GOOD: [u32; 6] = [1, 2, 3, 3, 4, 10];
	const RACES_EVIL: [u32; 7] = [1, 2, 2, 2, 3, 5, 10];

	let total_worth_of_good: u32 = good
		.split(' ')
		.map(|string| string.parse::<u32>().unwrap())
		.enumerate()
		.map(|(i, count)| RACES_GOOD[i] * count)
		.sum();
	let total_worth_of_evil: u32 = evil
		.split(' ')
		.map(|string| string.parse::<u32>().unwrap())
		.enumerate()
		.map(|(i, count)| RACES_EVIL[i] * count)
		.sum();

	if total_worth_of_good > total_worth_of_evil {
		String::from("Battle Result: Good triumphs over Evil")
	}
	else if total_worth_of_good < total_worth_of_evil {
		String::from("Battle Result: Evil eradicates all trace of Good")
	}
	else {
		String::from("Battle Result: No victor on this battle field")
	}
}

#[test]
fn returns_expected() {
	assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"), "Battle Result: Good triumphs over Evil");
	assert_eq!(good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"), "Battle Result: Evil eradicates all trace of Good");
	assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"), "Battle Result: No victor on this battle field");
}
