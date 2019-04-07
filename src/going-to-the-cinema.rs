fn movie(card: i32, ticket: i32, percent: f64) -> i32 {
	let mut count = 1;
	let mut ticket_price = ticket as f64 * percent;
	let mut card_price = card as f64 + ticket_price;

	while card_price.ceil() as i32 >= ticket * count {
		ticket_price *= percent;
		card_price += ticket_price;
		count += 1;
	}

	count
}

fn dotest(card: i32, ticket: i32, perc: f64, exp: i32) -> () {
	println!(" card: {:?};", card);
	println!("ticket: {:?};", ticket);
	println!("perc: {:?};", perc);
	let ans = movie(card, ticket, perc);
	println!("actual:\n{:?};", ans);
	println!("expect:\n{:?};", exp);
	println!(" {};", ans == exp);
	assert_eq!(ans, exp);
	println!("{};", "-");
}

#[test]
fn basic_tests() {
	// dotest(500, 15, 0.9, 43);
	// dotest(100, 10, 0.95, 24);
	dotest(0, 10, 0.95, 2);
}
