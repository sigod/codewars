fn abbrev_name(name: &str) -> String {
	let space_position = name.find(' ').unwrap() + 1;
	format!("{}.{}", &name[0..1], &name[space_position..space_position + 1])
		.to_uppercase()
}

#[test]
fn sample_tests() {
	assert_eq!(abbrev_name("Sam Harris"), "S.H");
	assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
	assert_eq!(abbrev_name("Evan Cole"), "E.C");
	assert_eq!(abbrev_name("P Favuzzi"), "P.F");
	assert_eq!(abbrev_name("David Mendieta"), "D.M");
}
