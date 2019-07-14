use std::fmt;

fn balance(book: &str) -> String {
    let book = Book::parse(&sanitize(book));

    book.generate_report()
}

fn sanitize(content: &str) -> String {
	let mut result = String::new();

	for c in content.chars() {
		if c.is_ascii_alphanumeric() || c == '\n' || c == ' ' || c == '.' {
			result.push(c);
		}
	}

	result
}

fn precision(value: f64) -> f64 {
	(value * 100_f64).round() / 100_f64
}

struct Check {
	number: i32,
	category: String,
	amount: f64,
}

impl Check {
	fn parse(content: &str) -> Check {
		let mut split = content.split_whitespace();

		Check {
			number: split.next().unwrap().parse().unwrap(),
			category: split.next().unwrap().to_owned(),
			amount: split.next().unwrap().parse().unwrap(),
		}
	}
}

impl fmt::Display for Check {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:03} {} {:.2}", self.number, self.category, self.amount)
	}
}

struct Book {
	original_balance: f64,
	checks: Vec<Check>,
}

impl Book {
	fn parse(content: &str) -> Book {
		let mut lines = content.lines();

		let original_balance: f64;
		loop {
			let line = lines.next().unwrap();

			if line != "" {
				original_balance = line.parse().unwrap();
				break;
			}
		}

		let mut checks = Vec::new();

		for line in lines {
			if line == "" {
				continue;
			}

			checks.push(Check::parse(line));
		}

		Book {
			original_balance,
			checks,
		}
	}

	fn generate_report(&self) -> String {
		let mut result = String::new();

		result.push_str(&format!("Original Balance: {:.2}\n", self.original_balance));

		let mut balance = self.original_balance;
		let mut expense = 0.0;

		for check in self.checks.iter() {
			expense += check.amount;
			balance -= check.amount;

			result.push_str(&format!("{} Balance {:.2}\n", check, precision(balance)));
		}

		result.push_str(&format!("Total expense  {:.2}\n", precision(expense)));

		let average_expense = expense / self.checks.len() as f64;
		result.push_str(&format!("Average expense  {:.2}", precision(average_expense)));

		result
	}
}

#[cfg(test)]

mod tests {
    use super::*;
    fn dotest(book: &str, exp: &str) -> () {
        println!("book:{}", book);
        let ans = balance(book);
        println!("actual:\n{}", ans);
        println!("expect:\n{}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
let b1 = r#"
1000.00!=

125 Market !=:125.45
126 Hardware =34.95
127 Video! 7.45
128 Book :14.32
129 Gasoline ::16.10
"#;
let b2 = r#"
1233.00
125 Hardware;! 24.8?;
123 Flowers 93.5
127 Meat 120.90
120 Picture 34.00
124 Gasoline 11.00
123 Photos;! 71.4?;
122 Picture 93.5
132 Tyres;! 19.00,?;
129 Stamps 13.6
129 Fruits{} 17.6
129 Market;! 128.00?;
121 Gasoline;! 13.6?;
"#;

let b1sol="Original Balance: 1000.00\n125 Market 125.45 Balance 874.55\n126 Hardware 34.95 Balance 839.60\n127 Video 7.45 Balance 832.15\n128 Book 14.32 Balance 817.83\n129 Gasoline 16.10 Balance 801.73\nTotal expense  198.27\nAverage expense  39.65";
let b2sol="Original Balance: 1233.00\n125 Hardware 24.80 Balance 1208.20\n123 Flowers 93.50 Balance 1114.70\n127 Meat 120.90 Balance 993.80\n120 Picture 34.00 Balance 959.80\n124 Gasoline 11.00 Balance 948.80\n123 Photos 71.40 Balance 877.40\n122 Picture 93.50 Balance 783.90\n132 Tyres 19.00 Balance 764.90\n129 Stamps 13.60 Balance 751.30\n129 Fruits 17.60 Balance 733.70\n129 Market 128.00 Balance 605.70\n121 Gasoline 13.60 Balance 592.10\nTotal expense  640.90\nAverage expense  53.41";

            dotest(b1, b1sol);
            dotest(b2, b2sol);
    }
}
