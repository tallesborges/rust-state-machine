use std::collections::BTreeMap;

type AccountId = String;
type Balance = u128;

#[derive(Debug)]
pub struct Pallet {
	balances: BTreeMap<AccountId, Balance>,
}

impl Pallet {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
		self.balances.insert(who.to_string(), amount);
	}

	pub fn balance(&self, who: &AccountId) -> Balance {
		*self.balances.get(who).unwrap_or(&0)
	}

	pub fn transfer(&mut self, caller: AccountId, to: AccountId, amount: Balance) -> Result<(), &'static str> {
		let caller_balance = self.balance(&caller);
		let to_balance = self.balance(&to);

		let new_caller_balance = caller_balance.checked_sub(amount).ok_or("Not enough funds.")?;
		let new_to_balance = to_balance.checked_add(amount).ok_or("Not enough funds.")?;

		self.set_balance(&caller, new_caller_balance);
		self.set_balance(&to, new_to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances = super::Pallet::new();

		assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 100), Err("Not enough funds."));

		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 10), Ok(()));
		assert_eq!(balances.balance(&"alice".to_string()), 90);
		assert_eq!(balances.balance(&"bob".to_string()), 10);

		assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 91), Err("Not enough funds."));
	}
}
