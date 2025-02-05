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

#[derive(Debug)]
#[macros::runtime]
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
				call: RuntimeCall::balances(balances::Call::transfer { to: bob.clone(), amount: 30 }),
			},
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::balances(balances::Call::transfer { to: charlie.clone(), amount: 30 }),
			},
		],
	};

	runtime.execute_block(block_1).expect("invalid block");
	assert_eq!(runtime.system.block_number(), 1);
	assert_eq!(runtime.balances.balance(&alice), 40);
	assert_eq!(runtime.balances.balance(&bob), 30);
	assert_eq!(runtime.balances.balance(&charlie), 30);

	let block_2 = types::Block {
		header: support::Header { block_number: 2 },
		extrinsics: vec![
			support::Extrinsic {
				caller: bob.clone(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim { claim: "content" }),
			},
			support::Extrinsic {
				caller: charlie.clone(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim { claim: "content" }),
			},
		],
	};

	runtime.execute_block(block_2).expect("invalid block");
	assert_eq!(runtime.system.block_number(), 2);
	assert_eq!(runtime.proof_of_existence.get_claim(&"content"), Some(&bob));

	println!("{:#?}", runtime);
}
