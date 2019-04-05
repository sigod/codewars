fn fcn(n: i32) -> i64 {
	2i64.pow(n as u32)
}

fn testequal(n: i32, exp: i64) -> () {
	assert_eq!(exp, fcn(n))
}
#[test]
fn basics() {
	testequal(17, 131072);
	testequal(21, 2097152);
}
