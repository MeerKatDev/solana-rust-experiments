use pinocchio::sysvars::Sysvar;
use pinocchio::{
    ProgramResult, account_info::AccountInfo, instruction::Signer, program_error::ProgramError,
    pubkey::find_program_address, seeds, sysvars::rent::Rent,
};
use pinocchio_system::instructions::CreateAccount;

/// Processes the `CreateAccount` instruction.
///
/// ### Parameters:
/// - `accounts`: The accounts required for the instruction.
/// - `lamports`: The number of lamports to transfer to the new account.
/// - `space`: The number of bytes to allocate for the new account.
/// - `owner`: The program that will own the new account.
/// - `signers`: The signers array needed to authorize the transaction.
///
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The funding and owner account.
/// 1. `[WRITE, SIGNER]` The new account to be created.
pub fn process(
    accounts: &[AccountInfo],
    _lamports: u64, // Number of lamports to transfer to the new account.
    _space: u64,    // Number of bytes to allocate for the new account.
) -> ProgramResult {
    let seed: &str = "state_seed";
    // Accounts passed to the instruction
    let owner_account = &accounts[0]; // The account that will fund the new account.
    let new_account = &accounts[1]; // The new account that will be created.

    // Ensure the funding account and new account are signers
    if !owner_account.is_signer() || !new_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let owner = owner_account.key();

    let seeds = &[seed.as_bytes(), owner.as_ref()];
    // derive the canonical bump during account init
    let (pda, bump) = find_program_address(seeds, &crate::ID);
    if pda.ne(new_account.key()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    let bump_binding = [bump];
    let signer_seeds = seeds!(seed.as_bytes(), owner.as_ref(), &bump_binding);
    let signers = [Signer::from(&signer_seeds[..])];

    CreateAccount {
        from: owner_account,
        to: new_account,
        lamports: Rent::get()?.minimum_balance(512usize),
        space: 512u64,
        owner: &crate::ID,
    }
    .invoke_signed(&signers)?;

    Ok(())
}
