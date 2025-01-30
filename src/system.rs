use std::{collections::BTreeMap, ops::AddAssign};

use num::{One, Zero};

#[derive(Debug)]
pub struct Pallet<AccountId, Nonce, BlockNumber> {
	block_number: BlockNumber,
	nonce: BTreeMap<AccountId, Nonce>,
}

impl<AccountId, Nonce, BlockNumber> Pallet<AccountId, Nonce, BlockNumber>
where
	AccountId: Ord + Clone,
	Nonce: Zero + One + Copy,
	BlockNumber: Zero + One + Copy + AddAssign,
{
	pub fn new() -> Self {
		Self { block_number: BlockNumber::zero(), nonce: BTreeMap::default() }
	}

	pub fn block_number(&self) -> BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number += BlockNumber::one();
	}

	pub fn inc_nonce(&mut self, who: &AccountId) {
		let who_nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());

		self.nonce.insert(who.clone(), who_nonce + Nonce::one());
	}
}

#[cfg(test)]
mod test {

	#[test]
	fn init_system() {
		let mut system = super::Pallet::<String, u32, u32>::new();
		system.inc_block_number();
		system.inc_nonce(&"alice".to_string());

		assert_eq!(system.block_number, 1);
		assert_eq!(system.nonce.get("alice"), Some(&1));
		assert_eq!(system.nonce.get("bob"), None);
	}
}
