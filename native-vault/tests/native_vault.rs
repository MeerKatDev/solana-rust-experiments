use mollusk_svm::{Mollusk, result::Check};
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct VaultInstruction {
    pub discriminator: u8,
    pub lamports: u64,
    pub space: u64,
}

#[test]
fn create_account_test() {
    let program_id: Pubkey = native_vault::ID.into();
    println!("program_id {:?}", program_id);
    let owner = Pubkey::new_unique();
    // casual wallet public key
    let new_key = Pubkey::new_unique();

    let (system_program, system_account) = mollusk_svm::program::keyed_account_for_system_program();

    let owner_account = Account::new(1_000_000_000, 0, &system_program);
    let new_account = Account::new(0, 0, &system_program);

    // instruction - offchain representation
    let instruction_off = VaultInstruction {
        discriminator: 0,
        lamports: 10_000,
        space: 512,
    };

    //
    let mut instr_in_bytes: Vec<u8> = Vec::new();
    instruction_off.serialize(&mut instr_in_bytes).unwrap();

    // instruction - onchain representation
    let instruction_on = Instruction::new_with_bytes(
        program_id,
        &instr_in_bytes,
        vec![
            // signers
            AccountMeta::new(owner, true),
            AccountMeta::new(new_key, true),
            // system program for system instruction
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    let mollusk = Mollusk::new(&program_id, "target/deploy/native_vault");

    let checks = vec![Check::success()];

    let accounts = vec![
        (owner, owner_account),
        (new_key, new_account),
        (system_program, system_account),
    ];

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

#[test]
fn create_pda_test() {
    let program_id: Pubkey = native_vault::ID.into();
    println!("program_id {:?}", program_id);
    let owner = Pubkey::new_unique();
    // it cannot be any key
    // let new_key = Pubkey::new_unique();

    let (system_program, system_account) = mollusk_svm::program::keyed_account_for_system_program();

    let owner_account = Account::new(1_000_000_000, 0, &system_program);

    // PDA derived from owner
    let (new_key, _new_acc_bump) =
        Pubkey::find_program_address(&[(b"state_seed"), owner.as_ref()], &program_id);
    let new_account = Account::new(0, 0, &system_program);

    // instruction - offchain representation
    let instruction_off = VaultInstruction {
        discriminator: 1,
        lamports: 10_000,
        space: 512,
    };

    //
    let mut instr_in_bytes: Vec<u8> = Vec::new();
    instruction_off.serialize(&mut instr_in_bytes).unwrap();

    // instruction - onchain representation
    let instruction_on = Instruction::new_with_bytes(
        program_id,
        &instr_in_bytes,
        vec![
            // signers
            AccountMeta::new(owner, true),
            AccountMeta::new(new_key, true),
            // system program for system instruction
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    let mollusk = Mollusk::new(&program_id, "target/deploy/native_vault");

    let checks = vec![Check::success()];

    let accounts = vec![
        (owner, owner_account),
        (new_key, new_account),
        (system_program, system_account),
    ];

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
