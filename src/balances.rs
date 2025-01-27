use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
	balances: BTreeMap<String, u128>,
}

impl Pallet {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &str, amount: u128) {
		self.balances.insert(who.to_string(), amount);
	}

	pub fn balance(&self, who: &str) -> u128 {
		*self.balances.get(who).unwrap_or(&0)
	}

	pub fn transfer(&mut self, caller: &str, to: &str, amount: u128) -> Result<(), &'static str> {
		let caller_balance = self.balance(caller);
		let to_balance = self.balance(to);

		let new_caller_balance = caller_balance.checked_sub(amount).ok_or("Not enough funds.")?;
		let new_to_balance = to_balance.checked_add(amount).ok_or("Not enough funds.")?;

		self.set_balance(caller, new_caller_balance);
		self.set_balance(to, new_to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::new();

		assert_eq!(balances.balance("alice"), 0);
		balances.set_balance("alice", 100);
		assert_eq!(balances.balance("alice"), 100);
		assert_eq!(balances.balance("bob"), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances = super::Pallet::new();

		assert_eq!(balances.transfer("alice", "bob", 100), Err("Not enough funds."));

		balances.set_balance("alice", 100);
		assert_eq!(balances.transfer("alice", "bob", 10), Ok(()));
		assert_eq!(balances.balance("alice"), 90);
		assert_eq!(balances.balance("bob"), 10);

		assert_eq!(balances.transfer("alice", "bob", 91), Err("Not enough funds."));
	}
}
