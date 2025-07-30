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

#[test]
fn deposit_test() {
    let program_id: Pubkey = native_vault::ID.into();
    let signer_key = Pubkey::new_unique();
    let (system_program, system_account) = mollusk_svm::program::keyed_account_for_system_program();

    // Derive the vault PDA from the signer's key
    let (vault_key, _bump) =
        Pubkey::find_program_address(&[ixs::deposit::SEED, signer_key.as_ref()], &program_id);

    let initial_signer_balance = 1_000_000_000u64;
    let initial_vault_balance = 0u64;
    let lamports_to_deposit = 100_000u64;

    let signer_account = Account::new(initial_signer_balance, 0, &system_program);
    let vault_account = Account::new(initial_vault_balance, 0, &program_id); // Owned by your program

    let instruction_off = VaultInstruction {
        discriminator: VaultInstructions::Deposit.into(),
        lamports: lamports_to_deposit,
        space: 0, // Not used for Deposit
    };

    let mut instr_in_bytes: Vec<u8> = Vec::new();
    instruction_off.serialize(&mut instr_in_bytes).unwrap();

    let instruction_on = Instruction::new_with_bytes(
        program_id,
        &instr_in_bytes,
        vec![
            AccountMeta::new(signer_key, true), // signer
            AccountMeta::new(vault_key, false), // vault PDA (must match derived PDA)
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    let mollusk = Mollusk::new(&program_id, "target/deploy/native_vault");

    let checks = vec![
        Check::success(),
        Check::account(&signer_key)
            .lamports(initial_signer_balance - lamports_to_deposit)
            .build(),
        Check::account(&vault_key)
            .lamports(initial_vault_balance + lamports_to_deposit)
            .build(),
    ];

    let accounts = vec![
        (signer_key, signer_account),
        (vault_key, vault_account),
        (system_program, system_account),
    ];

    let _ = mollusk.process_and_validate_instruction(&instruction_on, &accounts, &checks);
}

#[test]
fn withdraw_test() {
    let program_id: Pubkey = native_vault::ID.into();
    let signer_key = Pubkey::new_unique();
    let (system_program, system_account) = mollusk_svm::program::keyed_account_for_system_program();

    // Derive the PDA vault address from the signer key
    let (vault_key, _bump) =
        Pubkey::find_program_address(&[ixs::withdraw::SEED, signer_key.as_ref()], &program_id);

    let lamports_in_vault = 200_000u64;
    let lamports_to_withdraw = 100_000u64;
    let initial_signer_balance = 1_000_000u64;

    let vault_account = Account::new(lamports_in_vault, 0, &program_id);
    let signer_account = Account::new(initial_signer_balance, 0, &system_program);

    let instruction_off = VaultInstruction {
        discriminator: VaultInstructions::Withdraw.into(),
        lamports: lamports_to_withdraw,
        space: 0, // Not used in Withdraw
    };

    let mut instr_in_bytes: Vec<u8> = Vec::new();
    instruction_off.serialize(&mut instr_in_bytes).unwrap();

    let instruction_on = Instruction::new_with_bytes(
        program_id,
        &instr_in_bytes,
        vec![
            AccountMeta::new(vault_key, false),
            AccountMeta::new(signer_key, true),
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    let mollusk = Mollusk::new(&program_id, "target/deploy/native_vault");

    let checks = vec![
        Check::success(),
        Check::account(&vault_key)
            .lamports(lamports_in_vault - lamports_to_withdraw)
            .build(),
        Check::account(&signer_key)
            .lamports(initial_signer_balance + lamports_to_withdraw)
            .build(),
    ];

    let accounts = vec![
        (vault_key, vault_account),
        (signer_key, signer_account),
        (system_program, system_account),
    ];

    let _ = mollusk.process_and_validate_instruction(&instruction_on, &accounts, &checks);
}
