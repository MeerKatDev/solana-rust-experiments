use borsh::{BorshDeserialize, BorshSerialize};
use mollusk_svm::{Mollusk, result::Check, sysvar::Sysvars};
use native_vault::{ixs, ixs::VaultInstructions};
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct VaultInstruction {
    pub discriminator: u8,
    pub lamports: u64,
    pub space: u64,
}

#[test]
fn create_account_test() {
    let program_id: Pubkey = native_vault::ID.into();
    let owner_key = Pubkey::new_unique();
    // casual wallet public key
    let new_account_key = Pubkey::new_unique();

    let (system_program, system_account) = mollusk_svm::program::keyed_account_for_system_program();

    let owner_account = Account::new(1_000_000_000, 0, &system_program);
    // it is a wallet account, so it does not need to be derived
    let new_account = Account::new(0, 0, &system_program);

    let space_required = 512u64;
    let additional_lamports = 10_000;

    // instruction - offchain representation
    let instruction_off = VaultInstruction {
        discriminator: VaultInstructions::CreateAccount.into(), // CreateAccount (non-pda)
        lamports: additional_lamports,
        space: space_required,
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
            AccountMeta::new(owner_key, true),
            AccountMeta::new(new_account_key, true),
            // system program for system instruction
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    let mollusk = Mollusk::new(&program_id, "target/deploy/native_vault");

    let rent = Sysvars::default().rent;
    let rent_exempt_min = rent.minimum_balance(space_required as usize);

    let checks = vec![
        Check::success(),
        Check::account(&new_account_key)
            .owner(&program_id)
            .space(space_required as usize)
            .lamports(rent_exempt_min + additional_lamports)
            .build(),
    ];

    let accounts = vec![
        (owner_key, owner_account),
        (new_account_key, new_account),
        (system_program, system_account),
    ];

    let _ = mollusk.process_and_validate_instruction(&instruction_on, &accounts, &checks);
}

#[test]
fn create_pda_test() {
    let program_id: Pubkey = native_vault::ID.into();
    println!("program_id {:?}", program_id);
    let owner = Pubkey::new_unique();

    let (system_program, system_account) = mollusk_svm::program::keyed_account_for_system_program();

    let owner_account = Account::new(1_000_000_000, 0, &system_program);

    // PDA derived from owner
    let (new_account_key, _new_acc_bump) =
        Pubkey::find_program_address(&[ixs::create_pda::SEED, owner.as_ref()], &program_id);

    let new_account = Account::new(0, 0, &system_program);

    let space_required = 512u64;
    let additional_lamports = 10_000;

    // instruction - offchain representation
    let instruction_off = VaultInstruction {
        discriminator: VaultInstructions::CreatePdaAccount.into(),
        lamports: additional_lamports,
        space: space_required,
    };

    // serializing manually
    let mut instr_in_bytes: Vec<u8> = Vec::new();
    instruction_off.serialize(&mut instr_in_bytes).unwrap();

    // instruction - onchain representation
    let instruction_on = Instruction::new_with_bytes(
        program_id,
        &instr_in_bytes,
        vec![
            // signers
            AccountMeta::new(owner, true),
            AccountMeta::new(new_account_key, true),
            // system program for system instruction
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    let mollusk = Mollusk::new(&program_id, "target/deploy/native_vault");

    let checks = vec![Check::success()];

    let accounts = vec![
        (owner, owner_account),
        (new_account_key, new_account),
        (system_program, system_account),
    ];

    // Here we're using result account API instead of Checks
    let result = mollusk.process_and_validate_instruction(&instruction_on, &accounts, &checks);
    let updated_account = result.get_account(&new_account_key).unwrap();

    let rent = Sysvars::default().rent;
    let rent_exempt_min = rent.minimum_balance(space_required as usize);

    assert_eq!(
        updated_account.lamports,
        rent_exempt_min + additional_lamports
    ); // rent exempt + 10 000
}
