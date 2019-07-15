fn boolean_to_string(b: bool) -> String {
	match b {
		true => "true".to_owned(),
		false => "false".to_owned(),
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(boolean_to_string(true), "true", "When we pass in true, we want the string \"true\" as output");
        assert_eq!(boolean_to_string(false), "false", "When we pass in false, we want the string \"false\" as output");
        assert_eq!(boolean_to_string(false), "false", "When we pass in false, we want the string \"false\" as output");
    }
}
