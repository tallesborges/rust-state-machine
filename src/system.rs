use std::{collections::BTreeMap, ops::AddAssign};

use num::{One, Zero};

#[derive(Debug)]
pub struct Pallet<T: Config> {
	block_number: T::BlockNumber,
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

pub trait Config {
	type AccountId: Ord + Clone;
	type Nonce: Zero + One + Copy;
	type BlockNumber: Zero + One + Copy + AddAssign;
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::default() }
	}

	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}

	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let who_nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());

		self.nonce.insert(who.clone(), who_nonce + T::Nonce::one());
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type AccountId = String;
		type Nonce = u32;
		type BlockNumber = u32;
	}

	#[test]
	fn init_system() {
		let mut system = super::Pallet::<TestConfig>::new();
		system.inc_block_number();
		system.inc_nonce(&"alice".to_string());

		assert_eq!(system.block_number, 1);
		assert_eq!(system.nonce.get("alice"), Some(&1));
		assert_eq!(system.nonce.get("bob"), None);
	}
}
