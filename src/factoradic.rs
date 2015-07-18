/// An instance of Factoradic
/// represents a number in the
/// factorial number system. It stores
/// its digits in a big-endian way in
/// a fixed-size (at runtime) Vec<usize>
///
/// A number represented by a Factoradic
/// is often useful because it can
/// easily be turned into a permutation
/// of n elements, where n is the
/// number of digits in the Factoradic
///
/// For more details, see:
///
/// http://en.wikipedia.org/wiki/Factorial_number_system
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Factoradic {
	pub digits: Vec<usize>,
	pub has_overflowed: bool
}

use masked_vec;
use std::ops;

#[allow(dead_code)]
impl Factoradic {
	pub fn new(size: usize) -> Factoradic {

		let mut result = Factoradic {
			digits: Vec::with_capacity(size),
			has_overflowed: false
		};

		for _ in 0..size {
			result.digits.push(0);
		}

		result
	}

	pub fn from_vec(digits: Vec<usize>) -> Option<Factoradic> {

		let size = digits.len();

		for (i, digit) in digits.iter().enumerate() {
			let place = size - i - 1;
			if *digit > place {
				return None;
			}
		}

		return Some(Factoradic {
			digits: digits,
			has_overflowed: false
		})
	}

	pub fn incr(&mut self) -> &mut Factoradic {

		let size = self.digits.len();

		// i is the index into digits
		// digit is digits[i]
		// place is n in "n's place" (1's place, 2's place, etc)
		let mut has_overflowed = true;
		for (i, digit) in self.digits.iter_mut().enumerate().rev() {
			let place = size - i - 1;
			if *digit == place {
				*digit = 0;
			}
			else {
				*digit += 1;
				has_overflowed = false;
				break;
			}
		}

		if has_overflowed {
			self.has_overflowed = true;
		}

		self
	}

	pub fn to_u64(&self) -> u64 {
		let mut factorial: u64 = 1;
		let mut sum: u64 = 0;

		for (place, digit) in self.digits.iter().rev().enumerate() {
			if place != 0 {
				factorial *= place as u64;
			}
			sum += (*digit as u64) * factorial;
		}

		sum
	}

	pub fn to_permutation(&self) -> Vec<usize> {
		let unpermuted_indices: ops::Range<usize> = 0..(self.digits.len());
		let mut masked_indices: masked_vec::MaskedVec<usize> = masked_vec::MaskedVec::from_vec(unpermuted_indices.collect());
		let mut result = Vec::<usize>::with_capacity(self.digits.len());

		for f_digit in self.digits.iter() {
			let f_digit_u = *f_digit as usize;
			let masked_digit = masked_indices.get_at(f_digit_u);
			result.push(masked_digit);
			masked_indices.mask_at(f_digit_u);
		}

		result
	}
}

pub fn new(size: usize) -> Factoradic {
	Factoradic::new(size)
}

#[test]
fn test_incr() {

	let mut actual: Factoradic = Factoradic::new(3);

	let check = |expected: Vec<usize>, actual: &Factoradic| {
		let expected_fact: Factoradic = Factoradic::from_vec(expected).unwrap();
		assert_eq!(expected_fact, *actual);
	};

	check(vec![0, 0, 0], &actual);
	actual.incr();
	check(vec![0, 1, 0], &actual);
	actual.incr();
	check(vec![1, 0, 0], &actual);
	actual.incr();
	check(vec![1, 1, 0], &actual);
	actual.incr();
	check(vec![2, 0, 0], &actual);
	actual.incr();
	check(vec![2, 1, 0], &actual);

}

#[test]
fn test_to_u64() {
	let check = |expected: u64, digits: Vec<usize>| {
		assert_eq!(expected, Factoradic::from_vec(digits).unwrap().to_u64());
	};

	check(0, vec![0, 0, 0]);
	check(1, vec![0, 1, 0]);
	check(2, vec![1, 0, 0]);
	check(3, vec![1, 1, 0]);
	check(4, vec![2, 0, 0]);
	check(5, vec![2, 1, 0]);
}

#[test]
fn test_from_vec() {
	assert_eq!(
		Some(Factoradic {
			digits: vec![5, 4, 3, 2, 1, 0],
			has_overflowed: false
		}),
		Factoradic::from_vec(vec![5, 4, 3, 2, 1, 0])
	);
	assert_eq!(
		None,
		Factoradic::from_vec(vec![1])
	);
	assert_eq!(
		None,
		Factoradic::from_vec(vec![2, 0])
	);
	assert_eq!(
		None,
		Factoradic::from_vec(vec![3, 0, 0])
	);
	assert_eq!(
		None,
		Factoradic::from_vec(vec![4, 0, 0, 0])
	);
	assert_eq!(
		None,
		Factoradic::from_vec(vec![5, 0, 0, 0, 0])
	);
}

#[test]
fn test_to_permutation() {
	let check = |expected: Vec<usize>, actual: Vec<usize>| {
		assert_eq!(expected, Factoradic::from_vec(actual).unwrap().to_permutation())
	};

	check(vec![0, 1, 2, 3], vec![0, 0, 0, 0]);

	check(vec![0, 1, 2], vec![0, 0, 0]);
	check(vec![0, 2, 1], vec![0, 1, 0]);
	check(vec![1, 0, 2], vec![1, 0, 0]);
	check(vec![1, 2, 0], vec![1, 1, 0]);
	check(vec![2, 0, 1], vec![2, 0, 0]);
	check(vec![2, 1, 0], vec![2, 1, 0]);
}