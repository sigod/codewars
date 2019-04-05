fn num_as_roman(num: i32) -> String {
	if num < 0 {
		panic!();
	}

	let num = num as usize;
	let units = vec!["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
	let dozens = vec!["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
	let hundreds = vec!["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];

	"M".repeat(num / 1000)
		+ hundreds[num % 1000 / 100]
		+ dozens[num % 100 / 10]
		+ units[num % 10]
}

#[test]
fn returns_expected() {
	assert_eq!(num_as_roman(182), "CLXXXII");
	assert_eq!(num_as_roman(1990), "MCMXC");
	assert_eq!(num_as_roman(1875), "MDCCCLXXV");
}
