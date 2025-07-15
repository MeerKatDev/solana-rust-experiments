use mollusk_svm::Mollusk;
use solana_sdk::{
	account::Account, 
	instruction::{AccountMeta, Instruction}, 
	pubkey::Pubkey
};

use counter_mollusk::Counter;
use borsh::{BorshSerialize, BorshDeserialize};

#[test]
fn test_addition_instruction() {
	// unique program id
	let program_id = Pubkey::new_unique();
	// unique account
	let key1 = Pubkey::new_unique();

	// random number to start the count from.
	let count = 10;

	// instruction - offchain representation
	let instruction_off = Counter { count };

	// instruction - onchain representation
	let instruction_on = Instruction::new_with_borsh(
	    program_id,
	    &instruction_off,
	    // cannot be readonly
	    // but it can be not the signer
	    vec![AccountMeta::new(key1, false)],
	);

	//
    let mut instr_in_bytes: Vec<u8> = Vec::new();
    instruction_off.serialize(&mut instr_in_bytes).unwrap();

    let mut account1 = Account::default();
    // need to set the owner otherwise it cannot access the program
    account1.owner = program_id;
    // this contains the borsh
    account1.data = instr_in_bytes;

    // NOTE: doesn't want to work?
    // apparently it doesn't see the borsh instruction
    // account1.data = instruction_off.try_to_vec_with_schema().unwrap();

    let accounts = vec![
        (key1, account1)
    ];

    // loads the program as Mollusk "Virtual machine/runtime" (dunno how to call it)
	let mollusk = Mollusk::new(&program_id, "target/deploy/counter_mollusk");

	// Execute the ONCHAIN instruction, and insert the accounts with initial state we set.
	let result = mollusk.process_instruction(&instruction_on, &accounts);
	// AFTER execution, let's get the resulting data
	let updated_account_data = &result.resulting_accounts[0].1.data;
	// deserializing into a new counter struct for readability 
	// (we could have used the binary data though)
	let updated_counter = Counter::try_from_slice(updated_account_data).unwrap();
	// confirming that the counter was incremented by 1
	// so the transaction was executed.
	assert_eq!(updated_counter.count, count + 1);
}