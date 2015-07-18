
use factoradic;

pub fn new<'a, T>(vec: &'a Vec<T>) -> LexicographicIterator<'a, T> {
	LexicographicIterator::new(vec)
}

pub struct LexicographicIterator<'a, T: 'a> {
	inital_state: &'a Vec<T>,
	current_factoradic: factoradic::Factoradic
}

impl <'a, T> LexicographicIterator<'a, T> {
	pub fn new(vec: &'a Vec<T>) -> LexicographicIterator<'a, T> {
		LexicographicIterator {
			current_factoradic: factoradic::new(vec.len()),
			inital_state: vec
		}
	}
}

impl <'a, T> Iterator for LexicographicIterator<'a, T> {
	type Item = Vec<&'a T>;

	fn next(&mut self) -> Option<Vec<&'a T>> {
		if self.current_factoradic.has_overflowed {
			None
		}
		else {
			let mut result = Vec::with_capacity(self.current_factoradic.digits.len());

			for index in self.current_factoradic.to_permutation() {
				result.push(&self.inital_state[index]);
			}
			self.current_factoradic.incr();

			Some(result)
		}
	}
}

#[test]
fn test_permutations() {
	let vals = vec!['a', 'b', 'c'];
	let mut pi = LexicographicIterator::new(&vals);

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
