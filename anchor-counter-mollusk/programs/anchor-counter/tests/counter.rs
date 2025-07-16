use mollusk_svm::Mollusk;

use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use anchor_lang::{system_program, AccountSerialize, AnchorDeserialize, InstructionData, Key};

use anchor_counter::{
    instruction::{Increment, Initialize},
    Counter, ID as PROGRAM_ID,
};

#[test]
fn test_counter_increment() {
    let signer_pubkey = Pubkey::new_unique();
    let (counter_pda, _bump) = Pubkey::find_program_address(
        &[
            b"counter",
            signer_pubkey.key().as_ref(), // user = payer = signer
        ],
        &PROGRAM_ID,
    );

    // Create account owned by the program with initial data
    let mut system_account = Account::default();
    system_account.executable = true;

    let mut signer_account = Account::default();
    signer_account.owner = system_program::ID;

    let mut counter_account = Account::default();
    counter_account.owner = PROGRAM_ID;

    let counter = Counter { count: 0 };
    let mut counter_data = vec![];
    // already inputs into `counter_data` discriminant + INIT_SPACE bytes
    counter.try_serialize(&mut counter_data).unwrap();

    counter_account.data = counter_data;
    counter_account.lamports = 1_000_000;

    let accounts = vec![
        AccountMeta::new(counter_pda, false),
        AccountMeta::new(signer_pubkey, true),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let init_ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: accounts.clone(),
        // gives the instruction discriminant
        data: Initialize {}.data(),
    };

    let all_accounts = vec![
        (counter_pda, counter_account),
        (signer_pubkey, signer_account),
        (system_program::ID, system_account),
    ];

    // Create Mollusk runtime
    let mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/anchor_counter");

    // Process instruction with initial accounts
    let result_init = mollusk.process_instruction(&init_ix, &all_accounts);
    let initialized_accounts = result_init.resulting_accounts;

    let incr_accounts = vec![AccountMeta::new(counter_pda, false)];

    let incr_ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: incr_accounts,
        // this is simply increment discriminator
        data: Increment {}.data(),
    };

    let result = mollusk.process_instruction(&incr_ix, &initialized_accounts);

    // Deserialize updated data after execution
    // This contains the discriminant + serialized content
    let updated_counter_data = &result.resulting_accounts[0].1.data;

    // The data includes discriminator + count, skip discriminator to deserialize Counter
    let updated_counter = Counter::try_from_slice(&updated_counter_data[8..]).unwrap();

    // Counter should be incremented by 1
    assert_eq!(updated_counter.count, 1);
}
