// This Kata is disappointing... Both in technical details: old Rust version, forced poor Ast implementation. And difficulty.

use std::collections::HashMap;
use std::mem;

const SUBTYPE_IMMUTABLE: &'static str = "imm";
const SUBTYPE_ARGUMENT: &'static str = "arg";

#[derive(Clone, Debug)]
enum Ast {
	BinOp(String, Box<Ast>, Box<Ast>),
	UnOp(String, i32),
}

impl Ast {
	fn new_operator(operator: &str, left: Self, right: Self) -> Self {
		Ast::BinOp(operator.to_owned(), Box::new(left), Box::new(right))
	}

	fn new_argument(index: usize) -> Self {
		Ast::UnOp(SUBTYPE_ARGUMENT.to_owned(), index as i32)
	}

	fn new_immutable(value: i32) -> Self {
		Ast::UnOp(SUBTYPE_IMMUTABLE.to_owned(), value)
	}

	fn new_single(token: &str, arguments: &HashMap<&str, usize>) -> Self {
		if let Ok(value) = token.parse::<i32>() {
			Self::new_immutable(value)
		}
		else {
			Self::new_argument(*arguments.get(token).unwrap())
		}
	}

	fn is_immutable(string: &str) -> bool {
		string == SUBTYPE_IMMUTABLE
	}

	fn from_expression(tokens: &[String], arguments: &HashMap<&str, usize>) -> (Self, usize, bool) {
		let mut consumed_count = 0;
		let mut node;
		let mut is_strong_node;

		if tokens[0] == "(" {
			consumed_count += 1;

			let (new_node, count, _) = Self::from_expression(&tokens[consumed_count..], arguments);
			node = new_node;
			consumed_count += count;
			is_strong_node = true;
		}
		else {
			node = Self::new_single(&tokens[0], arguments);
			consumed_count += 1;
			is_strong_node = false;
		}

		while consumed_count < tokens.len() {
			if tokens[consumed_count] == ")" {
				consumed_count += 1;
				is_strong_node = true;
				break;
			}

			let (new_node, count) = Self::from_expression_part(node, is_strong_node, &tokens[consumed_count..], arguments);
			node = new_node;
			consumed_count += count;
			is_strong_node = false;
		}

		(node, consumed_count, is_strong_node)
	}

	fn priority(&self) -> i32 {
		match self {
			&Ast::BinOp(ref operator, _, _) => operator_priority(&operator),
			&Ast::UnOp(_, _) => 0,
		}
	}

	fn is_binary(&self) -> bool {
		match self {
			&Ast::BinOp(_, _, _) => true,
			&Ast::UnOp(_, _) => false,
		}
	}

	fn right_mut<'a>(&'a mut self) -> &'a mut Ast {
		match self {
			&mut Ast::BinOp(_, _, ref mut right) => &mut *right,
			&mut Ast::UnOp(_, _) => panic!(),
		}
	}

	fn from_expression_part(mut left_node: Self, is_strong_node: bool, tokens: &[String], arguments: &HashMap<&str, usize>) -> (Self, usize) {
		assert!(is_operator(&tokens[0]), "Expected operator, but found: {}", tokens[0]);
		assert!(!is_operator(&tokens[1]), "Expected operand, but found: {}", tokens[1]);

		let token_count;
		let result;
		let right_node;

		if tokens[1] == "(" {
			let (node, count, _) = Self::from_expression(&tokens[1..], arguments);

			token_count = count + 1;
			right_node = node;
		}
		else {
			token_count = 2;
			right_node = Self::new_single(&tokens[1], arguments);
		}

		if left_node.is_binary() && !is_strong_node && left_node.priority() < operator_priority(&tokens[0]) {
			let left = mem::replace(left_node.right_mut(), Self::new_immutable(0));
			let mut new_node = Self::new_operator(&tokens[0], left, right_node);

			mem::swap(left_node.right_mut(), &mut new_node);

			result = left_node;
		}
		else {
			result = Self::new_operator(&tokens[0], left_node, right_node);
		}

		(result, token_count)
	}

	fn from_tokens(tokens: &[String], arguments: &HashMap<&str, usize>) -> Self {
		let (node, count, _) = Self::from_expression(tokens, arguments);

		assert_eq!(tokens.len(), count, "Parsing didn't consume all tokens.");

		node
	}
}

struct Instruction {
}

#[allow(non_snake_case)]
impl Instruction {
	fn IM(value: i32) -> String {
		format!("IM {}", value)
	}

	fn AR(id: i32) -> String {
		format!("AR {}", id)
	}

	fn SW() -> String {
		"SW".to_owned()
	}

	fn PU() -> String {
		"PU".to_owned()
	}

	fn PO() -> String {
		"PO".to_owned()
	}

	fn AD() -> String {
		"AD".to_owned()
	}

	fn SU() -> String {
		"SU".to_owned()
	}

	fn MU() -> String {
		"MU".to_owned()
	}

	fn DI() -> String {
		"DI".to_owned()
	}
}

struct Compiler {
}

impl Compiler {
	fn new() -> Compiler {
		Compiler {}
	}

	fn tokenize<'a>(&self, program: &'a str) -> Vec<String> {
		let mut tokens: Vec<String> = vec![];

		let mut iter = program.chars().peekable();
		loop {
			match iter.peek() {
				Some(&c) => match c {
					'a'...'z'|'A'...'Z' => {
						let mut tmp = String::new();
						while iter.peek().is_some() && iter.peek().unwrap().is_alphabetic() {
							tmp.push(iter.next().unwrap());
						}
						tokens.push(tmp);
					},
					'0'...'9' => {
						let mut tmp = String::new();
						while iter.peek().is_some() && iter.peek().unwrap().is_numeric() {
							tmp.push(iter.next().unwrap());
						}
						tokens.push(tmp);
					},
					' ' => { iter.next(); },
					_ => {
						tokens.push(iter.next().unwrap().to_string());
					},
				},
				None => break
			}
		}

		tokens
	}

	fn compile(&mut self, program: &str) -> Vec<String> {
		let ast = self.pass1(program);
		let ast = self.pass2(&ast);
		self.pass3(&ast)
	}

	fn read_arguments<'a>(tokens: &'a Vec<String>) -> HashMap<&'a str, usize> {
		assert!(tokens.len() > 2, "Too few tokens in the program.");
		assert_eq!(tokens.first().unwrap(), "[", "The first token must be '['.");

		let mut arguments = HashMap::new();

		for (index, token) in tokens.iter().enumerate() {
			if token == "[" {
				continue;
			}
			if token == "]" {
				return arguments;
			}

			arguments.insert(token.as_str(), index - 1);
		}

		panic!("No ']' token found.");
	}

	fn pass1(&mut self, program: &str) -> Ast {
		let tokens = self.tokenize(program);
		let arguments = Compiler::read_arguments(&tokens);

		let header_size = arguments.len() + 2;
		let program = &tokens[header_size..];

		Ast::from_tokens(program, &arguments)
	}

	fn pass2(&mut self, ast: &Ast) -> Ast {
		match ast {
			&Ast::BinOp(ref operator, ref left, ref right) => {
				let new_left = self.pass2(left);
				let new_right = self.pass2(right);

				if let Ast::UnOp(ref subtype_left, ref value_left) = new_left {
					if let Ast::UnOp(ref subtype_right, ref value_right) = new_right {
						if Ast::is_immutable(subtype_left) && Ast::is_immutable(subtype_right) {
							let value = match operator.as_str() {
								"+" => value_left + value_right,
								"-" => value_left - value_right,
								"*" => value_left * value_right,
								"/" => value_left / value_right,
								_ => panic!("Unknown operator: {}", operator),
							};

							return Ast::new_immutable(value);
						}
					}
				}

				Ast::new_operator(operator, new_left, new_right)
			},
			&Ast::UnOp(_, _) => ast.clone(),
		}
	}

	fn pass3(&mut self, ast: &Ast) -> Vec<String> {
		match ast {
			&Ast::BinOp(ref operator, ref left, ref right) => {
				let mut code = self.pass3(&*right);
				code.push(Instruction::PU());

				code.append(&mut self.pass3(&*left));

				code.push(Instruction::SW());
				code.push(Instruction::PO());
				code.push(Instruction::SW());

				match operator.as_str() {
					"+" => code.push(Instruction::AD()),
					"-" => code.push(Instruction::SU()),
					"*" => code.push(Instruction::MU()),
					"/" => code.push(Instruction::DI()),
					_ => panic!("Unknown operator: {}", operator),
				}

				code
			},
			&Ast::UnOp(ref subtype, ref value) => {
				if Ast::is_immutable(subtype) {
					vec![Instruction::IM(*value)]
				}
				else {
					vec![Instruction::AR(*value)]
				}
			}
		}
	}
}

fn is_operator(string: &str) -> bool {
	string == "+" || string == "-" || string == "*" || string == "/"
}

fn operator_priority(operator: &str) -> i32 {
	match operator {
		"+" | "-" => 1,
		"*" | "/" => 2,
		_ => panic!(),
	}
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

fn simulate(assembly: Vec<String>, argv: Vec<i32>) -> i32 {
	let mut r = (0, 0);
	let mut stack: Vec<i32> = vec![];

	for ins in assembly {
		let mut ws = ins.split_whitespace();
		match ws.next() {
			Some("IM") => r.0 = i32::from_str_radix(ws.next().unwrap(), 10).unwrap(),
			Some("AR") => r.0 = argv[i32::from_str_radix(ws.next().unwrap(), 10).unwrap() as usize],
			Some("SW") => r = (r.1,r.0),
			Some("PU") => stack.push(r.0),
			Some("PO") => r.0 = stack.pop().unwrap(),
			Some("AD") => r.0 += r.1,
			Some("SU") => r.0 -= r.1,
			Some("MU") => r.0 *= r.1,
			Some("DI") => r.0 /= r.1,
			_ => panic!("Invalid instruction encountered"),
		}
	}

	r.0
}

#[test]
fn returns_expected() {
	{
		let ast = Compiler::new().pass1("[ a b ] a + b");
		assert_eq!(&format!("{:?}", ast), r#"BinOp("+", UnOp("arg", 0), UnOp("arg", 1))"#);
	}
	{
		let program = "[ a b c ] a + b * c";
		let expected = r#"BinOp("+", UnOp("arg", 0), BinOp("*", UnOp("arg", 1), UnOp("arg", 2)))"#;
		let ast = Compiler::new().pass1(program);
		assert_eq!(&format!("{:?}", ast), expected);
	}
	{
		let program = Compiler::new().compile("[ ] 1");
		assert_eq!(simulate(program, vec![]), 1);
	}
	{
		let program = Compiler::new().compile("[ a b c ] a + b * c");
		assert_eq!(simulate(program, vec![2, 2, 2]), 6);
	}
	{
		let program = Compiler::new().compile("[ a b c ] ( a + b ) * c");
		assert_eq!(simulate(program, vec![2, 2, 2]), 8);
	}
	{
		let program = Compiler::new().compile("[ x y z ] x - y - z + 10 / 5 / 2 - 7 / 1 / 7");
		assert_eq!(simulate(program, vec![2, 1, 1]), 0);
	}
}
