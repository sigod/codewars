const MAX_DISTANCE: u32 = 100;

fn riders(stations: &Vec<u32>) -> u32 {
	let mut rider = 1;
	let mut distance = 0;

	for delta_distance in stations.iter() {
		if distance + delta_distance > MAX_DISTANCE {
			distance = 0;
			rider += 1;
		}

		distance += delta_distance;
	}

	rider
}

#[test]
fn sample_tests() {
	assert_eq!(riders(&vec![18, 15]), 1);
	assert_eq!(riders(&vec![43, 23, 40, 13]), 2);
	assert_eq!(riders(&vec![33, 8, 16, 47, 30, 30, 46]), 3);
	assert_eq!(riders(&vec![6, 24, 6, 8, 28, 8, 23, 47, 17, 29, 37, 18, 40, 49]), 4);
}
