fn printer_error(string: &str) -> String {
	let error_count = string
		.chars()
		.filter(|&c| c as u8 > 'm' as u8)
		.count();

	format!("{}/{}", error_count, string.len())
}

#[test]
fn should_pass_all_the_tests_provided() {
	assert_eq!(&printer_error("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"), "3/56");
	assert_eq!(&printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"), "6/60");
	assert_eq!(&printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyzuuuuu"), "11/65");
}
