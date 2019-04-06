fn is_inside_circle(x: i32, y: i32, radius: i32) -> bool {
	let center_x = radius - 1;
	let center_y = radius - 1;

	let c2 = (center_x - x).pow(2) + (center_y - y).pow(2);
	let distance = (c2 as f32).sqrt();

	distance < radius as f32
}

fn circle(radius: i32) -> String {
	if radius < 0 {
		return "".to_string();
	}
	if radius == 0 {
		return "\n".to_string();
	}

	let diameter = radius * 2 - 1;
	let mut circle = String::with_capacity(((diameter + 1) * diameter) as usize);

	for y in 0..diameter {
		for x in 0..diameter {
			if is_inside_circle(x, y, radius) {
				circle.push('\u{2588}');
			}
			else {
				circle.push(' ');
			}
		}

		circle.push('\n');
	}

	circle
}

#[test]
fn basic_tests() {
	assert_eq!(circle(-1), "", "Negative radii should return the empty string.");
	assert_eq!(circle(-321), "", "Negative radii should return the empty string.");
	assert_eq!(circle(0), "\n", "A radius of 0 should produce \"\\n\"");
	assert_eq!(circle(1), "█\n");
	assert_eq!(circle(2), "███\n███\n███\n");
	assert_eq!(circle(10), "".to_owned() +
		"     █████████     \n" +
		"    ███████████    \n" +
		"  ███████████████  \n" +
		"  ███████████████  \n" +
		" █████████████████ \n" +
		"███████████████████\n" +
		"███████████████████\n" +
		"███████████████████\n" +
		"███████████████████\n" +
		"███████████████████\n" +
		"███████████████████\n" +
		"███████████████████\n" +
		"███████████████████\n" +
		"███████████████████\n" +
		" █████████████████ \n" +
		"  ███████████████  \n" +
		"  ███████████████  \n" +
		"    ███████████    \n" +
		"     █████████     \n"
	);
}
