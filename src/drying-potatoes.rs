fn potatoes (p0: i64, w0: i64, p1: i64) -> i64 {
    let matter_mass = 100 * w0 * (100 - p0);
    let w1 = matter_mass / (100 - p1);

    w1 / 100
}

fn dotest(p0: i64, w0: i64, p1: i64, exp: i64) -> () {
    let ans = potatoes(p0, w0, p1);
    assert_eq!(ans, exp)
}

#[test]
fn tests_potatoes() {
    dotest(99, 100, 98, 50);
    dotest(82, 127, 80, 114);
    dotest(82, 134, 77, 104);
}
