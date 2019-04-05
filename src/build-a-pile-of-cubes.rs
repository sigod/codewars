fn find_nb(m: u64) -> i32 {
	let mut n = 0;
	let mut mass = 0;

	while mass < m {
		n += 1;
		mass += (n as u64).pow(3);
	}

	if mass == m {n} else {-1}
}

fn testing(n: u64, exp: i32) -> () {
	assert_eq!(find_nb(n), exp);
}

#[test]
fn basics_find_nb() {
	testing(4183059834009, 2022);
	testing(24723578342962, -1);
	testing(135440716410000, 4824);
	testing(40539911473216, 3568);
	testing(26825883955641, 3218);
}
