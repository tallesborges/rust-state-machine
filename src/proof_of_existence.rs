use std::{collections::BTreeMap, fmt::Debug};

pub trait Config: crate::system::Config {
	type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}

	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		self.claims.get(claim)
	}
}

#[macros::call]
impl<T: Config> Pallet<T> {
	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> crate::support::DispatchResult {
		if self.claims.get(&claim).is_some() {
			return Err("Claim already exists");
		}
		self.claims.insert(claim, caller);
		Ok(())
	}

	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> crate::support::DispatchResult {
		let owner = self.claims.get(&claim).ok_or("claim does not exist")?;

		if owner != &caller {
			return Err("caller is not the owner");
		}

		self.claims.remove(&claim);

		Ok(())
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = &'static str;
		type Nonce = u32;
		type BlockNumber = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		let mut proof_of_exixtence = super::Pallet::<TestConfig>::new();
		// intial check
		assert_eq!(proof_of_exixtence.get_claim(&"none"), None);
		// create claim
		assert_eq!(proof_of_exixtence.create_claim("alice", "content"), Ok(()));
		// get claim
		assert_eq!(proof_of_exixtence.get_claim(&"content"), Some(&"alice"));
		// already exists
		assert_eq!(proof_of_exixtence.create_claim("bob", "content"), Err("Claim already exists"));
		// revoke
		assert_eq!(proof_of_exixtence.revoke_claim("alice", "content"), Ok(()));
		// invoke again
		assert_eq!(proof_of_exixtence.revoke_claim("alice", "content"), Err("claim does not exist"));
	}
}
