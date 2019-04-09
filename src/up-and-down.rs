fn arrange(string: &str) -> String {
	let mut words = string.trim().split_whitespace().collect::<Vec<_>>();

	let mut i = 0;
	loop {
		if i + 1 >= words.len() {
			break;
		}
		if words[i].len() > words[i + 1].len() {
			words.swap(i, i + 1);
		}

		if i + 2 >= words.len() {
			break;
		}
		if words[i + 1].len() < words[i + 2].len() {
			words.swap(i + 1, i + 2);
		}

		i += 2;
	}

	words
		.iter()
		.enumerate()
		.map(|(i, sub)| {
			if i % 2 == 0 {
				sub.to_lowercase()
			}
			else {
				sub.to_uppercase()
			}
		})
		.collect::<Vec<_>>()
		.join(" ")
}

fn testing(s: &str, exp: &str) -> () {
	assert_eq!(arrange(s), exp.to_string())
}

#[test]
fn basics_arrange() {
	testing("who hit retaining The That a we taken", "who RETAINING hit THAT a THE we TAKEN"); // 3
	testing("on I came up were so grandmothers", "i CAME on WERE up GRANDMOTHERS so"); // 4
}
