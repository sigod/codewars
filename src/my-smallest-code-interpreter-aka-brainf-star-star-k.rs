// https://www.codewars.com/kata/my-smallest-code-interpreter-aka-brainf-star-star-k

enum Command {
	Nop,
	MoveRight,
	MoveLeft,
	IncrementCell,
	DecrementCell,
	Output,
	Input,
	JumpForward(usize),
	JumpBackward(usize),
}

impl Command {
	fn parse(c: char) -> Command {
		match c {
			'>' => Command::MoveRight,
			'<' => Command::MoveLeft,
			'+' => Command::IncrementCell,
			'-' => Command::DecrementCell,
			'.' => Command::Output,
			',' => Command::Input,
			'[' => Command::JumpForward(0),
			']' => Command::JumpBackward(0),
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
}

impl Tape {
	fn new() -> Tape {
		Tape {
			tape_negative: Vec::new(),
			tape_positive: Vec::new(),
		}
	}

	fn get_cell(&mut self, cursor: isize) -> &mut u8 {
		if cursor >= 0 {
			let cursor = cursor as usize;

			while self.tape_positive.len() <= cursor {
				self.tape_positive.push(0);
			}

			&mut self.tape_positive[cursor]
		}
		else {
			let cursor = -(cursor + 1) as usize;

			while self.tape_negative.len() <= cursor {
				self.tape_negative.push(0);
			}

			&mut self.tape_negative[cursor]
		}
	}
}

struct BrainFuck {
	commands: Vec<Command>,
}

impl BrainFuck {
	fn parse(code: &str) -> BrainFuck {
		let mut commands = code.chars().map(Command::parse).filter(Command::is_meaningful).collect::<Vec<_>>();

		BrainFuck::init_jump_commands(&mut commands);

		BrainFuck {
			commands,
		}
	}

	fn init_jump_commands(commands: &mut Vec<Command>) {
		let mut stack = Vec::new();

		for index in 0..commands.len() {
			match commands[index] {
				Command::JumpForward(_) => {
					stack.push(index);
				},
				Command::JumpBackward(_) => {
					let last_index = stack.pop().unwrap();

					commands[index] = Command::JumpBackward(last_index);

					match commands[last_index] {
						Command::JumpForward(_) => {
							commands[last_index] = Command::JumpForward(index);
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

	fn run(&mut self, input: Vec<u8>) -> Vec<u8> {
		let mut output = Vec::new();
		let mut input_index = 0;

		let mut tape = Tape::new();
		let mut tape_cursor = 0;

		let mut commands_cursor = 0;

		while commands_cursor < self.commands.len() {
			match self.commands[commands_cursor] {
				Command::MoveRight => {
					tape_cursor += 1;
				},
				Command::MoveLeft => {
					tape_cursor -= 1;
				},
				Command::IncrementCell => {
					let mut cell = tape.get_cell(tape_cursor);
					*cell = if *cell == 255 {0} else {*cell + 1};
				},
				Command::DecrementCell => {
					let mut cell = tape.get_cell(tape_cursor);
					*cell = if *cell == 0 {255} else {*cell - 1};
				},
				Command::Output => {
					let mut cell = tape.get_cell(tape_cursor);
					output.push(*cell);
				},
				Command::Input => {
					let mut cell = tape.get_cell(tape_cursor);
					*cell = input[input_index];
					input_index += 1;
				},
				Command::JumpForward(target_index) => {
					let cell = *tape.get_cell(tape_cursor);
					if cell == 0 {
						commands_cursor = target_index;
					}
				},
				Command::JumpBackward(target_index) => {
					let cell = *tape.get_cell(tape_cursor);
					if cell != 0 {
						commands_cursor = target_index;
					}
				},
				Command::Nop => {},
			};

			commands_cursor += 1;
		}


		output
	}
}

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
	let mut script = BrainFuck::parse(code);
	let output = script.run(input);
	output
}

fn main() {
}

#[test]
fn example_tests() {
  // Echo until byte 255 encountered
  // assert_eq!(String::from_utf8(brain_luck(",+[-.,+]", ez_vec("Codewars", 255))).unwrap(), "Codewars");
  // Echo until byte 0 encountered
  // assert_eq!(String::from_utf8(brain_luck(",[.[-],]", ez_vec("Codewars", 0))).unwrap(), "Codewars");
  //
  assert_eq!(brain_luck(",+[-.,+]", vec![0, 1, 2, 255]), vec![0, 1, 2]);
  // Multiply two numbers
  assert_eq!(brain_luck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![8, 9]), vec![72]);
}
