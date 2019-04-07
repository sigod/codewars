enum Command {
	Nop,
	MoveRight,
	MoveLeft,
	FlipBit,
	Output,
	Input,
	JumpRight(usize),
	JumpLeft(usize),
}

impl Command {
	fn parse(c: char) -> Command {
		match c {
			'>' => Command::MoveRight,
			'<' => Command::MoveLeft,
			'+' => Command::FlipBit,
			';' => Command::Output,
			',' => Command::Input,
			'[' => Command::JumpRight(0),
			']' => Command::JumpLeft(0),
			_ => Command::Nop,
		}
	}

	fn is_meaningful(&self) -> bool {
		match *self {
			Command::Nop => false,
			_ => true,
		}
	}
}

struct Tape {
	tape_negative: Vec<u8>,
	tape_positive: Vec<u8>,
	cursor: isize,
}

impl Tape {
	fn new() -> Tape {
		Tape {
			tape_negative: Vec::new(),
			tape_positive: vec![0],
			cursor: 0,
		}
	}

	fn shift_right(&mut self) {
		self.cursor += 1;

		if self.cursor >= 0 {
			while self.cursor as usize >= 8 * self.tape_positive.len() {
				self.tape_positive.push(0);
			}
		}
	}

	fn shift_left(&mut self) {
		self.cursor -= 1;

		if self.cursor < 0 {
			while -(self.cursor + 1) as usize >= 8 * self.tape_negative.len() {
				self.tape_negative.push(0);
			}
		}
	}

	fn flip_bit(&mut self) {
		let bit = 1 - self.get_bit();
		self.set_bit(bit);
	}

	fn set_bit(&mut self, bit: u8) {
		let cursor;
		let cell;

		if self.cursor >= 0 {
			cursor = self.cursor as usize;
			cell = &mut self.tape_positive[cursor / 8];
		}
		else {
			cursor = -(self.cursor + 1) as usize;
			cell = &mut self.tape_negative[cursor / 8];
		}

		if bit == 1 {
			*cell |= 0b0000_0001u8.rotate_left((cursor % 8) as u32);
		}
		else if bit == 0 {
			*cell &= 0b1111_1110u8.rotate_left((cursor % 8) as u32);
		}
		else {
			panic!();
		}
	}

	fn get_bit(&self) -> u8 {
		let cursor;
		let cell;

		if self.cursor >= 0 {
			cursor = self.cursor as usize;
			cell = self.tape_positive[cursor / 8];
		}
		else {
			cursor = -(self.cursor + 1) as usize;
			cell = self.tape_negative[cursor / 8];
		}

		let value = cell & 0b0000_0001u8.rotate_left((cursor % 8) as u32);

		if value == 0 {0} else {1}
	}
}

struct Code {
	commands: Vec<Command>,
	cursor: usize,
}

impl Code {
	fn parse(code: &str) -> Code {
		let mut commands = code.chars().map(Command::parse).filter(Command::is_meaningful).collect::<Vec<_>>();

		Code::init_jump_commands(&mut commands);

		Code {
			commands,
			cursor: 0,
		}
	}

	fn init_jump_commands(commands: &mut Vec<Command>) {
		let mut stack = Vec::new();

		for index in 0..commands.len() {
			match commands[index] {
				Command::JumpRight(_) => {
					stack.push(index);
				},
				Command::JumpLeft(_) => {
					let last_index = stack.pop().unwrap();

					commands[index] = Command::JumpLeft(last_index);

					match commands[last_index] {
						Command::JumpRight(_) => {
							commands[last_index] = Command::JumpRight(index);
						},
						_ => panic!(),
					};
				},
				_ => {},
			};
		}

		if !stack.is_empty() {
			panic!();
		}
	}

	fn next(&mut self) -> Option<&Command> {
		match self.commands.get(self.cursor) {
			Some(ref command) => {
				self.cursor += 1;

				Some(command)
			},
			None => None,
		}
	}

	fn jump_right(&mut self) {
		match self.commands.get(self.cursor - 1).unwrap() {
			Command::JumpRight(cursor) => {
				self.cursor = *cursor + 1;
			},
			_ => panic!(),
		};
	}

	fn jump_left(&mut self) {
		match self.commands.get(self.cursor - 1).unwrap() {
			Command::JumpLeft(cursor) => {
				self.cursor = *cursor + 1;
			},
			_ => panic!(),
		};
	}
}

struct Data {
	data: Vec<u8>,
	cursor: usize,
}

impl Data {
	fn new() -> Data {
		Data {
			data: Vec::new(),
			cursor: 0,
		}
	}

	fn from_existing(values: Vec<u8>) -> Data {
		Data {
			data: values,
			cursor: 0,
		}
	}

	fn increment_size(&mut self) {
		let size = ((self.cursor + 1) as f32 / 8f32).ceil() as usize;
		while size > self.data.len() {
			self.data.push(0);
		}
	}

	fn push_bit(&mut self, bit: u8) {
		self.increment_size();

		let cell = &mut self.data[self.cursor / 8];

		if bit == 1 {
			*cell |= 0b0000_0001u8.rotate_left((self.cursor % 8) as u32);
		}
		else if bit == 0 {
			*cell &= 0b1111_1110u8.rotate_left((self.cursor % 8) as u32);
		}
		else {
			panic!();
		}

		self.cursor += 1;
	}

	fn get_next_bit(&mut self) -> u8 {
		let cursor = self.cursor;
		self.cursor += 1;

		let cell = self.data[cursor / 8];

		let value = cell & 0b0000_0001u8.rotate_left((cursor % 8) as u32);

		if value == 0 {0} else {1}
	}
}

struct BoolFuck {
	code: Code,
	tape: Tape,
	input: Data,
	output: Data,
}

impl BoolFuck {
	fn new(code: &str, input: Vec<u8>) -> BoolFuck {
		BoolFuck {
			code: Code::parse(code),
			tape: Tape::new(),
			input: Data::from_existing(input),
			output: Data::new(),
		}
	}

	fn run(&mut self) {
		loop {
			match self.code.next() {
				Some(Command::MoveRight) => {
					self.tape.shift_right();
				},
				Some(Command::MoveLeft) => {
					self.tape.shift_left();
				},
				Some(Command::FlipBit) => {
					self.tape.flip_bit();
				},
				Some(Command::Output) => {
					self.output.push_bit(self.tape.get_bit());
				},
				Some(Command::Input) => {
					self.tape.set_bit(self.input.get_next_bit());
				},
				Some(Command::JumpRight(_)) => {
					if self.tape.get_bit() == 0 {
						self.code.jump_right();
					}
				},
				Some(Command::JumpLeft(_)) => {
					if self.tape.get_bit() == 1 {
						self.code.jump_left();
					}
				},
				Some(Command::Nop) => {
				},
				None => {
					break;
				},
			};
		}
	}
}

fn boolfuck(code: &str, input: Vec<u8>) -> Vec<u8> {
	let mut script = BoolFuck::new(code, input);
	script.run();

	script.output.data
}

#[test]
fn example_test_cases() {
	// Hello World Program taken from the official website
	assert_eq!(boolfuck(";;;+;+;;+;+;+;+;+;+;;+;;+;;;+;;+;+;;+;;;+;;+;+;;+;+;;;;+;+;;+;;;+;;+;+;+;;;;;;;+;+;;+;;;+;+;;;+;+;;;;+;+;;+;;+;+;;+;;;+;;;+;;+;+;;+;;;+;+;;+;;+;+;+;;;;+;+;;;+;+;+;", Vec::new()), b"Hello, world!\n", "Your interpreter did not work with the code example provided on the official website");
	// Echo until byte(0) encountered
	assert_eq!(boolfuck(">,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>;>;>;>;>;>;>;>;>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]", b"Codewars\x00".to_vec()), b"Codewars");
	// Two numbers multiplier
	assert_eq!(boolfuck(">,>,>,>,>,>,>,>,>>,>,>,>,>,>,>,>,<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]>[>]+<[+<]>>>>>>>>>[+]>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<<<<<<<<<[>]+<[+<]>>>>>>>>>[+]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]<<<<<<<<<<<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>;>;>;>;>;>;>;>;<<<<<<<<", vec![8, 9]), vec![72]);
}
