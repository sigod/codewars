fn interpreter(code: &str, iterations: usize,
               width: usize, height: usize) -> String
{
	let code = code.chars().collect::<Vec<_>>();

	let mut tape: Vec<bool> = vec![false; width * height];
	let mut x = 0;
	let mut y = 0;

	let mut command_index = 0;
	let mut command_count = 0;

	while command_index < code.len() && command_count < iterations {
		command_count += 1;

		match code[command_index] {
			'n' => {
				y = if y == 0 {height - 1} else {y - 1};
			},
			'e' => {
				x = if x == width - 1 {0} else {x + 1};
			},
			's' => {
				y = if y == height - 1 {0} else {y + 1};
			},
			'w' => {
				x = if x == 0 {width - 1} else {x - 1};
			},
			'*' => {
				tape[y * width + x] = !tape[y * width + x];
			},
			'[' => {
				if tape[y * width + x] == false {
					let mut brace_count = 0;

					loop {
						command_index += 1;

						match code[command_index] {
							'[' => {
								brace_count += 1;
							}
							']' => {
								if brace_count == 0 {
									break;
								}
								else {
									brace_count -= 1;
								}
							},
							_ => {},
						};
					}
				}
			},
			']' => {
				if tape[y * width + x] == true {
					let mut brace_count = 0;

					loop {
						command_index -= 1;

						match code[command_index] {
							']' => {
								brace_count += 1;
							}
							'[' => {
								if brace_count == 0 {
									break;
								}
								else {
									brace_count -= 1;
								}
							},
							_ => {},
						};
					}
				}
			},
			_ => {
				command_count -= 1;
			},
		}

		command_index += 1;
	}

	let mut result = String::with_capacity(width * height);

	for y in 0..height {
		if y != 0 {
			result.push_str("\r\n");
		}
		for x in 0..width {
			let c = if tape[y * width + x] {'1'} else {'0'};
			result.push(c);
		}
	}

	result
}
