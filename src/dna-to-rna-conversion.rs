fn dna_to_rna(dna: &str) -> String {
	dna.replace("T", "U")
}

#[test]
fn returns_expected() {
	assert_eq!(dna_to_rna("TTTT"), "UUUU");
	assert_eq!(dna_to_rna("GCAT"), "GCAU");
}
