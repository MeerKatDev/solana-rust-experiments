use mollusk_svm::Mollusk;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use anchor_counter::instruction::Increment;
use anchor_counter::instruction::Initialize;
use anchor_counter::Counter;
use anchor_lang::{AnchorSerialize, AnchorDeserialize};
use anchor_counter::accounts::Initialize as InitializeAccounts;

use anchor_counter::ID;
use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use solana_sdk::system_program;

#[test]
fn test_counter_increment() {
    let program_id = ID;
    let (counter_pda, _bump_seed) = Pubkey::find_program_address(&[b"counter"], &program_id);
    let signer_pubkey = Pubkey::new_unique();
    let initial_count: u64 = 10;
    // Serialize instruction data — this should be the *arguments* sent to your program.
    let counter = Counter { count: initial_count };
	// 2. Serialize Counter + discriminator into a data buffer
	let mut counter_data = vec![0u8; 8];
    let instruction_data = counter.try_to_vec().unwrap();
	counter_data.extend_from_slice(&instruction_data);  // serialized Counter


    // AnchorSerialize::serialize(&initial_count, &mut cursor).unwrap();
    counter.serialize(&mut &mut counter_data[8..]).unwrap();

    // Create account owned by the program with initial data
    let mut program_account = Account::default();
    program_account.owner = system_program::ID;

    let mut signer_account = Account::default();
    signer_account.owner = system_program::ID;
    // signer_account.owner = signer_pubkey;

    let mut counter_account = Account::default();
    counter_account.owner = system_program::ID;
    counter_account.data = counter_data;

    let accounts = vec![
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new(signer_pubkey, true),
        AccountMeta::new(counter_pda, false),
    ];

    let init_ix = Instruction {
        program_id: system_program::ID,
        // accounts,
        accounts: accounts.clone(),
        data: Initialize {}.data(),
    };

    let all_accounts = vec![
        (counter_pda, counter_account),
        (signer_pubkey, signer_account),
        (system_program::ID, program_account),
    ];

    let incr_accounts = vec![
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new(signer_pubkey, true),
        AccountMeta::new(counter_pda, false),
    ];

    let incr_ix = Instruction {
        program_id: system_program::ID,
        accounts: incr_accounts,
        data: Increment {}.data(),
    };

    // Create Mollusk runtime
    let mollusk = Mollusk::new(&program_id, "../../target/deploy/anchor_counter");

    // Process instruction with initial accounts
    let result_init = mollusk.process_instruction(&init_ix, &all_accounts);
    let initialized_accounts = result_init.resulting_accounts;
    let result = mollusk.process_instruction(&incr_ix, &initialized_accounts);

    // Deserialize updated data after execution
    let updated_counter_data = &result.resulting_accounts[0].1.data;
    println!("updated_counter_data: {:?}", &updated_counter_data[8..]);

    // The data includes discriminator + count, skip discriminator to deserialize Counter
    let updated_counter = Counter::try_from_slice(&updated_counter_data[8..]).unwrap();

    // Counter should be incremented by 1
    assert_eq!(updated_counter.count, initial_count + 1);
}
