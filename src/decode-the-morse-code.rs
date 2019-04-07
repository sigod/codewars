// Preloaded:
//
// struct MorseDecoder {
//     morse_code: HashMap<String, String>,
// }
//
// MorseDecoder::new() populates the morse_code map, e.g. ".-" -> "A".

impl MorseDecoder {
	fn decode_morse(&self, encoded: &str) -> String {
		let encoded = encoded.trim();

		let mut result = String::new();

		for word in encoded.split("   ") {
			for letter in word.split(" ") {
				if self.morse_code.contains_key(letter) {
					result.push_str(self.morse_code[letter].as_str());
				}
			}

			result.push(' ');
		}

		let new_length = result.len() - 1;
		result.truncate(new_length);

		result
	}
}

#[test]
fn test_hey_jude() {
	let decoder = MorseDecoder::new();
	assert_eq!(decoder.decode_morse(".... . -.--   .--- ..- -.. ."), "HEY JUDE");
}
