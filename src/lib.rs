mod factoradic;
mod masked_vec;
mod lexicographic_iterator;

pub fn lexicographically<'a, T>(vec: &'a Vec<T>) -> lexicographic_iterator::LexicographicIterator<'a, T> {
	lexicographic_iterator::new(vec)
}

#[test]
fn test_lexicographically() {
	let vals = vec!['a', 'b', 'c'];
	let mut pi = lexicographically(&vals);

	{
		let mut test = |expecteds: Vec<char>| {
			let actuals = pi.next().unwrap();
			for (expected, actual) in expecteds.iter().zip(actuals) {
				assert_eq!(*expected, *actual);
			}
		};

		test(vec!['a', 'b', 'c']);
		test(vec!['a', 'c', 'b']);
		test(vec!['b', 'a', 'c']);
		test(vec!['b', 'c', 'a']);
		test(vec!['c', 'a', 'b']);
		test(vec!['c', 'b', 'a']);
	}
	assert_eq!(None, pi.next());
}

