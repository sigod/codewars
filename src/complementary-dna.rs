fn dna_strand(dna: &str) -> String {
	let mut result = String::with_capacity(dna.len());
	for c in dna.chars() {
		match c {
			'A' => result.push('T'),
			'T' => result.push('A'),
			'C' => result.push('G'),
			'G' => result.push('C'),
			_ => panic!(),
		}
	}

	result
}

#[test]
fn returns_expected() {
	assert_eq!(dna_strand("AAAA"),"TTTT");
	assert_eq!(dna_strand("ATTGC"),"TAACG");
	assert_eq!(dna_strand("GTAT"),"CATA");
}
