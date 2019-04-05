fn dbl_linear(n_: u32) -> u32 {
	let n = n_ as usize;
	let mut u = vec![0; n + 2];
	u[0] = 1;

	let mut count = 0;
	let mut yi = 0;
	let mut zi = 0;

	while count <= n {
		let y = 2 * u[yi] + 1;
		let z = 3 * u[zi] + 1;

		count += 1;

		if y > z {
			u[count] = z;
			zi += 1;
		}
		else if y < z {
			u[count] = y;
			yi += 1;
		}
		else {
			u[count] = y;
			yi += 1;
			zi += 1;
		}
	}

	u[n]
}
