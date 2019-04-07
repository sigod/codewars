fn nb_months(old: i32, new: i32, saving: i32, percent: f64) -> (i32, i32) {
	const FIXED_LOSS: f64 = 0.5 / 100.0;

	let mut price_old = old as f64;
	let mut price_new = new as f64;
	let mut percent = percent / 100.0;
	let mut savings = 0.0;
	let mut month = 0;

	while price_new > price_old + savings {
		month += 1;
		if month % 2 == 0 {
			percent += FIXED_LOSS;
		}

		price_old *= 1.0 - percent;
		price_new *= 1.0 - percent;
		savings += saving as f64;
	}

	(month, (price_old + savings - price_new).round() as i32)
}

fn testing(old: i32, new: i32, saving: i32, perc: f64, exp: (i32, i32)) -> () {
	assert_eq!(nb_months(old, new, saving, perc), exp)
}

#[test]
fn basics_nb_months() {
	testing(2000, 8000, 1000, 1.5, (6, 766));
	testing(12000, 8000, 1000, 1.5 , (0, 4000));
	testing(8000, 12000, 500, 1.0, (8, 597));
	testing(18000, 32000, 1500, 1.25, (8, 332));
	testing(7500, 32000, 300, 1.55, (25, 122));
}
