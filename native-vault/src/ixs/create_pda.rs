use pinocchio::{
    ProgramResult,
    account_info::AccountInfo,
    instruction::Signer,
    program_error::ProgramError,
    pubkey::find_program_address,
    seeds,
    sysvars::{Sysvar, rent::Rent},
};
use pinocchio_system::instructions::CreateAccount;

pub const SEED: &[u8] = b"create_pda_seed";

/// Processes the `CreatePdaAccount` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `lamports`: The number of lamports to transfer to the new account.
/// - `space`: The number of bytes to allocate for the new account.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The funding and owner account.
/// 1. `[WRITE, SIGNER]` The new account to be created.
/// 1. `[NON-WRITE, NON-SIGNER]` System Account
pub fn process(
    accounts: &[AccountInfo],
    lamports: u64, // Number of lamports to transfer to the new account (additionally).
    space: u64,    // Number of bytes to allocate for the new account.
) -> ProgramResult {
    // Accounts passed to the instruction
    let [owner_account, new_account, _system_program] = accounts else {
        return Err(ProgramError::InvalidAccountData);
    };

    // Ensure the funding account and new account are signers
    if !owner_account.is_signer() || !new_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let owner_key = owner_account.key();

    let seeds = &[SEED, owner_key.as_ref()];

    let (pda, bump) = find_program_address(seeds, &crate::ID);

    if pda.ne(new_account.key()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    let bump_binding = [bump];
    let signer_seeds = seeds!(SEED, owner_key.as_ref(), &bump_binding);
    let signers = [Signer::from(&signer_seeds)];
    let lamports = Rent::get()?
        .minimum_balance(space as usize)
        .checked_add(lamports)
        .unwrap();

    let create_ix = CreateAccount {
        from: owner_account,
        to: new_account,
        lamports,
        space,
        owner: &crate::ID,
    };

    create_ix.invoke_signed(&signers)?;

    Ok(())
}
