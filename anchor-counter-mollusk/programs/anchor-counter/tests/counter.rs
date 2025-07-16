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
// use anchor_lang::AccountDeserialize;
use anchor_counter::ID as PROGRAM_ID;
use anchor_lang::InstructionData;
use anchor_lang::prelude::AccountInfo;
use anchor_lang::Key;
use anchor_lang::ToAccountMetas;
use solana_sdk::system_program;
use anchor_lang::Space;

use solana_sdk::epoch_schedule::Epoch;
#[test]
fn test_counter_increment() {
    let signer_pubkey = Pubkey::new_unique();
    let (counter_pda, _bump) = Pubkey::find_program_address(&[
        b"counter", 
        signer_pubkey.key().as_ref() // user = payer = signer
    ], &PROGRAM_ID);

    println!("signer pubkey: {:?}", signer_pubkey);
    println!("counter_pda pubkey: {:?}", counter_pda);
    println!("system_program::ID: {:?}", system_program::ID);

    // Create account owned by the program with initial data
    let mut program_account = Account::default();
    program_account.owner = system_program::ID;
    program_account.executable = true;

    let mut signer_account = Account::default();
    signer_account.owner = system_program::ID;

    let mut counter_account = Account::default();
    // Solves Error Code: AccountOwnedByWrongProgram. Error Number: 3007. Error Message: 
    // The given account is owned by a different program than expected.
    counter_account.owner = PROGRAM_ID; //system_program::ID; // NOT PROGRAM_ID yet
    let mut data = vec![0u8; 8 + Counter::INIT_SPACE];
    // let discriminator = <Counter as AccountDeserialize>::discriminator();

    // data[..8].copy_from_slice(&discriminator);
    let counter = Counter { count: 0 };
    counter.serialize(&mut &mut data[8..]).unwrap();
    counter_account.data = data; //vec![0u8; 8 + Counter::INIT_SPACE]; // allocate enough space (discriminator + u64)
    counter_account.lamports = 1_000_000;   

    let accounts = vec![
        AccountMeta::new(counter_pda, false),
        AccountMeta::new(signer_pubkey, true),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let init_ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: accounts.clone(),
        // this gives  InstructionFallbackNotFound
        // data: Initialize { }.try_to_vec().unwrap(), 
        data: Initialize { }.data(),
    };

    let all_accounts = vec![
        (counter_pda, counter_account),
        (signer_pubkey, signer_account),
        (system_program::ID, program_account),
    ];

    // Create Mollusk runtime
    let mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/anchor_counter");

    // Process instruction with initial accounts
    let result_init = mollusk.process_instruction(&init_ix, &all_accounts);
    let initialized_accounts = result_init.resulting_accounts;

    let incr_accounts = vec![
        AccountMeta::new(counter_pda, false),
    ];

    let incr_ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: incr_accounts,
        data: Increment {}.data(),
    };

    let result = mollusk.process_instruction(&incr_ix, &initialized_accounts);

    // Deserialize updated data after execution
    let updated_counter_data = &result.resulting_accounts[0].1.data;
    println!("updated_counter_data: {:?}", &updated_counter_data[8..]);

    // The data includes discriminator + count, skip discriminator to deserialize Counter
    let updated_counter = Counter::try_from_slice(&updated_counter_data[8..]).unwrap();

    // Counter should be incremented by 1
    assert_eq!(updated_counter.count, 1);
}
