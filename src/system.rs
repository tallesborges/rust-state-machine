use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
	block_number: u32,
	nonce: BTreeMap<String, u32>,
}

impl Pallet {
	pub fn new() -> Self {
		Self { block_number: 0, nonce: BTreeMap::default() }
	}

	pub fn block_number(&self) -> u32 {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number += 1;
	}

	pub fn inc_nonce(&mut self, who: &str) {
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
		system.inc_nonce("alice");

		assert_eq!(system.block_number, 1);
		assert_eq!(system.nonce.get("alice"), Some(&1));
		assert_eq!(system.nonce.get("bob"), None);
	}
}
