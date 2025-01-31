mod balances;
mod support;
mod system;

use crate::support::Dispatch;

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
}

pub enum RuntimeCall {
	BalancesTransfer { to: types::AccountId, amount: types::Balance },
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type Nonce = types::Nonce;
	type BlockNumber = types::BlockNumber;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl Runtime {
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}

	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		self.system.inc_block_number();

		if self.system.block_number() != block.header.block_number {
			return Err("Invalid block number");
		}

		for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
			self.system.inc_nonce(&caller);
			let _ = self.dispatch(caller, call).map_err(|e| {
				eprintln!("Extrinsic Error\n\tBlockNumber: {}\n\tExtrinsic Number: {}\n\tError: {}", block.header.block_number, i, e)
			});
		}

		Ok(())
	}
}

impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;

	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> support::DispatchResult {
		match call {
			RuntimeCall::BalancesTransfer { to, amount } => {
				self.balances.transfer(caller, to, amount)?;
			},
		}

		Ok(())
	}
}

fn main() {
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	runtime.balances.set_balance(&alice.clone(), 100);

	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	runtime.system.inc_nonce(&alice);
	let _ = runtime
		.balances
		.transfer(alice.clone(), bob.clone(), 30)
		.map_err(|e| eprintln!("{}", e));

	runtime.system.inc_nonce(&alice);
	let _ = runtime
		.balances
		.transfer(alice.clone(), charlie.clone(), 20)
		.map_err(|e| eprintln!("{}", e));

	println!("{:#?}", runtime);
}
