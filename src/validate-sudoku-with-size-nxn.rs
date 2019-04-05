struct Sudoku {
	data: Vec<Vec<u32>>,
}

impl Sudoku {
	fn is_valid(&self) -> bool {
		let n = self.data.len();

		if n == 0 {
			return false;
		}
		if (n as f32).sqrt().fract() != 0.0 {
			return false;
		}
		if self.data.iter().position(|ref row| row.len() != n) != None {
			return false;
		}

		let mut numbers = Vec::with_capacity(n);
		for _ in 0..n {
			numbers.push(0);
		}

		let count = |numbers: &mut Vec<usize>, i: usize, j: usize| {
			let value = self.data[i][j] as usize;
			if value == 0 || value > n {
				return true;
			}
			numbers[value - 1] = 1;
			false
		};

		let check = |numbers: &mut Vec<usize>| {
			if numbers.iter().sum::<usize>() != n {
				true
			}
			else {
				for i in 0..n {
					numbers[i] = 0;
				}
				false
			}
		};

		for i in 0..n {
			for j in 0..n {
				if count(&mut numbers, i, j) {
					return false;
				}
			}

			if check(&mut numbers) {
				return false;
			}
		}
		for i in 0..n {
			for j in 0..n {
				if count(&mut numbers, i, j) {
					return false;
				}
			}

			if check(&mut numbers) {
				return false;
			}
		}

		let n_sqrt = (n as f32).sqrt() as usize;

		for i in 0..n_sqrt {
			for j in 0..n_sqrt {
				for i_ in 0..n_sqrt {
					for j_ in 0..n_sqrt {
						if count(&mut numbers, i * n_sqrt + i_, j * n_sqrt + j_) {
							return false;
						}
					}
				}

				if check(&mut numbers) {
					return false;
				}
			}
		}

		true
	}
}
