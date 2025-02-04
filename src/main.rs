mod balances;
mod proof_of_existence;
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
	pub type Content = &'static str;
}

pub enum RuntimeCall {
	Balances(balances::Call<Runtime>),
	ProofOfExistence(proof_of_existence::Call<Runtime>),
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
	proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type Nonce = types::Nonce;
	type BlockNumber = types::BlockNumber;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
	type Content = types::Content;
}

impl Runtime {
	fn new() -> Self {
		Self {
			system: system::Pallet::new(),
			balances: balances::Pallet::new(),
			proof_of_existence: proof_of_existence::Pallet::new(),
		}
	}

	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		self.system.inc_block_number();

		if self.system.block_number() != block.header.block_number {
			return Err("Invalid block number");
		}

		for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
			self.system.inc_nonce(&caller);
			let _ = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlockNumber: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
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
			RuntimeCall::Balances(call) => self.balances.dispatch(caller, call),
			RuntimeCall::ProofOfExistence(call) => self.proof_of_existence.dispatch(caller, call),
		}
	}
}

fn main() {
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	// setup
	runtime.balances.set_balance(&alice.clone(), 100);

	let block_1 = types::Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::Transfer { to: bob.clone(), amount: 30 }),
			},
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie.clone(), amount: 30 }),
			},
		],
	};

	runtime.execute_block(block_1).expect("invalid block");

	assert_eq!(runtime.system.block_number(), 1);
	assert_eq!(runtime.balances.balance(&alice), 40);
	assert_eq!(runtime.balances.balance(&bob), 30);
	assert_eq!(runtime.balances.balance(&charlie), 30);

	println!("{:#?}", runtime);
}
