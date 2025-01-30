use std::collections::BTreeMap;

type AccountId = String;
type BlockNumber = u32;
type Nonce = u32;

#[derive(Debug)]
pub struct Pallet {
	block_number: BlockNumber,
	nonce: BTreeMap<AccountId, Nonce>,
}

impl Pallet {
	pub fn new() -> Self {
		Self { block_number: 0, nonce: BTreeMap::default() }
	}

	pub fn block_number(&self) -> BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number += 1;
	}

	pub fn inc_nonce(&mut self, who: &AccountId) {
		let who_nonce = self.nonce.get(who).unwrap_or(&0);

		self.nonce.insert(who.to_string(), who_nonce + 1);
	}
}

#[cfg(test)]
mod test {

	#[test]
	fn init_system() {
		let mut system = super::Pallet::new();
		system.inc_block_number();
		system.inc_nonce(&"alice".to_string());

		assert_eq!(system.block_number, 1);
		assert_eq!(system.nonce.get("alice"), Some(&1));
		assert_eq!(system.nonce.get("bob"), None);
	}
}
