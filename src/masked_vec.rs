

pub struct MaskedVec<T: Copy> {
	vec: Vec<T>,
	mask: Vec<bool>,
	length: usize
}

#[allow(dead_code)]
impl <T: Copy> MaskedVec<T> {

	pub fn from_vec(vec: Vec<T>) -> MaskedVec<T> {

		let mask = vec![false; vec.len()];

		MaskedVec {
			length: vec.len(),
			vec: vec,
			mask: mask
		}
	}

	fn get_actual_index(&self, i: usize) -> Option<usize> {

		if i >= self.length {
			return None
		}

		let mut unmasked_count: usize = 0;
		let mut total_count: usize = 0;

		for is_masked in self.mask.iter() {
			if !is_masked {
				if unmasked_count == i {
					return Some(total_count)
				}
				else {
					unmasked_count += 1;
				}
			}
			total_count += 1;
		}

		panic!("This code should be unreachable.")
	}

	pub fn get_at(&self, masked_index: usize) -> T {
		match self.get_actual_index(masked_index) {
			None => panic!("MaskedVec index out of range"),
			Some(actual_index) => self.vec[actual_index]
		}
	}

	pub fn set_at(&mut self, masked_index: usize, val: T) {
		match self.get_actual_index(masked_index) {
			None => panic!("MaskedVec index out of range"),
			Some(actual_index) => self.vec[actual_index] = val
		}
	}

	pub fn is_masked_at(&self, actual_index: usize) -> bool {
		self.mask[actual_index]
	}

	pub fn mask_at(&mut self, masked_index: usize) {
		match self.get_actual_index(masked_index) {
			None => panic!("MaskedVec index out of range"),
			Some(actual_index) => {
				self.mask[actual_index] = true;
				self.length -= 1;
			}
		}
	}

	pub fn mask_at_actual(&mut self, actual_index: usize) {
		if !self.mask[actual_index] {
			self.mask[actual_index] = true;
			self.length -= 1;
		}
	}

	pub fn unmask_at_actual(&mut self, actual_index: usize) {
		if self.mask[actual_index] {
			self.mask[actual_index] = false;
			self.length += 1;
		}
	}
}

#[test]
fn test_from_vec() {
	let mvec = MaskedVec::from_vec(vec![1,2,3,4]);
	assert_eq!(vec![1,2,3,4], mvec.vec);
	assert_eq!(vec![false, false, false, false], mvec.mask);
	assert_eq!(4, mvec.length);
}

#[test]
fn test_get_actual_index() {
	let mut mvec = MaskedVec::from_vec(vec![1,2,3,4]);
	mvec.mask_at_actual(1);
	assert_eq!(2, mvec.get_actual_index(1).unwrap());
	assert_eq!(3, mvec.get_actual_index(2).unwrap());

	mvec.mask_at_actual(2);
	assert_eq!(3, mvec.get_actual_index(1).unwrap());

	assert_eq!(None, mvec.get_actual_index(2));
	assert_eq!(None, mvec.get_actual_index(3));
}

#[test]
fn test_mask_at() {
	let mut mvec = MaskedVec::from_vec(vec![1,2,3,4]);
	assert_eq!(vec![1,2,3,4], mvec.vec);
	assert_eq!(vec![false, false, false, false], mvec.mask);
	assert_eq!(4, mvec.length);

	mvec.mask_at(0);
	assert_eq!(vec![1,2,3,4], mvec.vec);
	assert_eq!(vec![true, false, false, false], mvec.mask);
	assert_eq!(3, mvec.length);

	mvec.mask_at(0);
	assert_eq!(vec![1,2,3,4], mvec.vec);
	assert_eq!(vec![true, true, false, false], mvec.mask);
	assert_eq!(2, mvec.length);

	mvec.mask_at(0);
	assert_eq!(vec![1,2,3,4], mvec.vec);
	assert_eq!(vec![true, true, true, false], mvec.mask);
	assert_eq!(1, mvec.length);

	mvec.mask_at(0);
	assert_eq!(vec![1,2,3,4], mvec.vec);
	assert_eq!(vec![true, true, true, true], mvec.mask);
	assert_eq!(0, mvec.length);
}

#[test]
fn integration_test() {
	let mut mvec = MaskedVec::from_vec(vec![1,2,3,4]);
	assert_eq!(vec![1,2,3,4], mvec.vec);
	assert_eq!(vec![false, false, false, false], mvec.mask);
	assert_eq!(4, mvec.length);

	// test that maskAt works
	mvec.mask_at_actual(2);
	assert_eq!(vec![1,2,3,4], mvec.vec);
	assert_eq!(vec![false, false, true, false], mvec.mask);
	assert_eq!(3, mvec.length);

	// test that maskAt is idempotent
	mvec.mask_at_actual(2);
	assert_eq!(vec![1,2,3,4], mvec.vec);
	assert_eq!(vec![false, false, true, false], mvec.mask);
	assert_eq!(3, mvec.length);

	// test is_masked_at
	assert_eq!(false, mvec.is_masked_at(0));
	assert_eq!(false, mvec.is_masked_at(1));
	assert_eq!(true, mvec.is_masked_at(2));
	assert_eq!(false, mvec.is_masked_at(3));

	// test get_at
	assert_eq!(1, mvec.get_at(0));
	assert_eq!(2, mvec.get_at(1));
	assert_eq!(4, mvec.get_at(2));

	// test set_at
	mvec.set_at(2, 40);
	assert_eq!(vec![1,2,3,40], mvec.vec);
	assert_eq!(vec![false, false, true, false], mvec.mask);
	assert_eq!(3, mvec.length);

	// test unmask
	mvec.unmask_at_actual(2);
	assert_eq!(vec![1,2,3,40], mvec.vec);
	assert_eq!(vec![false, false, false, false], mvec.mask);
	assert_eq!(4, mvec.length);

	// test unmask is idempotent
	mvec.unmask_at_actual(2);
	assert_eq!(vec![1,2,3,40], mvec.vec);
	assert_eq!(vec![false, false, false, false], mvec.mask);
	assert_eq!(4, mvec.length);
}
