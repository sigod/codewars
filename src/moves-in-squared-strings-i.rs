fn hor_mirror(s: String) -> String {
	let mut result = String::with_capacity(s.len());

	for row in s.split('\n').rev() {
		result.push_str(row);
		result.push('\n');
	}

	if !result.is_empty() {
		let new_length = result.len() - 1;
		result.truncate(new_length);
	}

	result
}

fn vert_mirror(s: String) -> String {
	let mut result = String::with_capacity(s.len());

	for row in s.split('\n') {
		for c in row.chars().rev() {
			result.push(c);
		}

		result.push('\n');
	}

	if !result.is_empty() {
		let new_length = result.len() - 1;
		result.truncate(new_length);
	}

	result
}

fn oper<F>(f: F, s: String) -> String
	where F: FnOnce(String) -> String {
	f(s)
}

fn testing1(s: &str, exp: &str) -> () {
	assert_eq!(oper(hor_mirror, s.to_string()), exp)
}
fn testing2(s: &str, exp: &str) -> () {
	assert_eq!(oper(vert_mirror, s.to_string()), exp)
}

#[test]
fn basics_oper() {
	testing1("lVHt\nJVhv\nCSbg\nyeCt", "yeCt\nCSbg\nJVhv\nlVHt");
	testing1("njMK\ndbrZ\nLPKo\ncEYz", "cEYz\nLPKo\ndbrZ\nnjMK");
	testing1("QMxo\ntmFe\nWLUG\nowoq", "owoq\nWLUG\ntmFe\nQMxo");

	testing2("hSgdHQ\nHnDMao\nClNNxX\niRvxxH\nbqTVvA\nwvSyRu", "QHdgSh\noaMDnH\nXxNNlC\nHxxvRi\nAvVTqb\nuRySvw");
	testing2("IzOTWE\nkkbeCM\nWuzZxM\nvDddJw\njiJyHF\nPVHfSx", "EWTOzI\nMCebkk\nMxZzuW\nwJddDv\nFHyJij\nxSfHVP");
	testing2("cuQW\nxOuD\nfZwp\neqFx", "WQuc\nDuOx\npwZf\nxFqe");
}
