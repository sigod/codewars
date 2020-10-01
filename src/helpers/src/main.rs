
fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("\tRust\tAll");

	let mut rust_total = 0;
	let mut all_total = 0;

	for kyu in (1..=8).rev() {
		let rust = check_rust(kyu)?;
		let all = check_all(kyu)?;

		println!("{} kyu\t{}\t{}", kyu, rust, all);

		rust_total += rust;
		all_total += all;
	}

	println!("Total\t{}\t{}", rust_total, all_total);

	Ok(())
}

fn check_rust(kyu: u8) -> Result<usize, Box<dyn std::error::Error>> {
	let url = format!("https://www.codewars.com/kata/search/rust?q=&r[]=-{}&beta=false", kyu);

	let text = reqwest::blocking::get(&url)?
		.text()?;

	parse_kata_count(&text)
}

fn check_all(kyu: u8) -> Result<usize, Box<dyn std::error::Error>> {
	let url = format!("https://www.codewars.com/kata/search/?q=&r[]=-{}&beta=false", kyu);

	let text = reqwest::blocking::get(&url)?
		.text()?;

	parse_kata_count(&text)
}

fn parse_kata_count(text: &str) -> Result<usize, Box<dyn std::error::Error>> {
	let right_index = text.find(" Kata Found").unwrap();
	let left_index = 1 + text[.. right_index].rfind(">").unwrap();

	let value = &text[left_index .. right_index];

	Ok(value.parse()?)
}
