fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
	// let mut result = Vec::new();

	// for value in a.iter() {
	// 	b.iter().find(|&&v| v == value)
	// }

	// result

	a.into_iter().filter(|ref a| b.iter().find(|ref b| **a == ***b) == None).collect()
}

#[test]
fn returns_expected() {
    assert_eq!(array_diff(vec![1,2], vec![1]), vec![2]);
    assert_eq!(array_diff(vec![1,2,2], vec![1]), vec![2,2]);
    assert_eq!(array_diff(vec![1,2,2], vec![2]), vec![1]);
    assert_eq!(array_diff(vec![1,2,2], vec![]), vec![1,2,2]);
    assert_eq!(array_diff(vec![], vec![1,2]), vec![]);
}
