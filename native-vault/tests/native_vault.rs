use mollusk_svm::{Mollusk, result::Check};
use solana_instruction::{AccountMeta, Instruction};
use solana_account::Account;
use solana_pubkey::Pubkey;

use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct VaultInstruction {
    pub discriminator: u8,
    pub lamports: u64,
    pub space: u64
}

#[test]
fn integration_test() {
	let program_id: Pubkey = native_vault::ID.into();
	println!("program_id {:?}", program_id);
	let owner = Pubkey::new_unique();
	let new_key = Pubkey::new_unique();

	// instruction - offchain representation
	let instruction_off = VaultInstruction { 
		discriminator: 0, 
		lamports: 10_000, 
		space: 512
	};

	//
    let mut instr_in_bytes: Vec<u8> = Vec::new();
    instruction_off.serialize(&mut instr_in_bytes).unwrap();

	// instruction - onchain representation
	let instruction_on = Instruction::new_with_bytes(
	    program_id,
	    &instr_in_bytes,
	    // cannot be readonly
	    // but it can be not the signer
	    vec![
	    	AccountMeta::new(owner, true), 
	    	AccountMeta::new(new_key, true)
	    ],
	);


    let mut owner_account = Account::default();
    // need to set the owner otherwise it cannot access the program
    owner_account.owner = program_id;

    let mut new_account = Account::default();
    // need to set the owner otherwise it cannot access the program
    new_account.owner = owner;

    let accounts = vec![
        (owner, owner_account),
        (new_key, new_account)
    ];

    // loads the program as Mollusk "Virtual machine/runtime" (dunno how to call it)
	let mollusk = Mollusk::new(&program_id, "target/deploy/native_vault");

    let checks = vec![Check::success()];

    let result = mollusk.process_and_validate_instruction(&instruction_on, &accounts, &checks);
	// AFTER execution, let's get the resulting data
	// let updated_account_data = &result.resulting_accounts[0].1.data;
	// // deserializing into a new counter struct for readability 
	// // (we could have used the binary data though)
	// let updated_counter = Counter::try_from_slice(updated_account_data).unwrap();
	// // confirming that the counter was incremented by 1
	// // so the transaction was executed.
	// assert_eq!(updated_counter.count, count + 1);
}