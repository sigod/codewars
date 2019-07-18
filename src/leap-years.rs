fn is_leap_year(year: i32) -> bool {
	year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn sample_tests() {
	assert_eq!(is_leap_year(1234), false);
	assert_eq!(is_leap_year(1984), true);
	assert_eq!(is_leap_year(2000), true);
	assert_eq!(is_leap_year(2010), false);
	assert_eq!(is_leap_year(2013), false);
}
