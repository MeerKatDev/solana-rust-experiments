use pinocchio::{
    ProgramResult, account_info::AccountInfo, instruction::Signer, program_error::ProgramError,
};

use pinocchio::pubkey::try_find_program_address;
use pinocchio::seeds;

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
/// 0. `[WRITE, SIGNER]` The funding account.
/// 1. `[WRITE, SIGNER]` The new account to be created.
pub fn process(
    accounts: &[AccountInfo],
    lamports: u64,               // Number of lamports to transfer to the new account.
    space: u64,                  // Number of bytes to allocate for the new account.
    owner_account: &AccountInfo, // Pubkey of the program that will own the new account.
) -> ProgramResult {
    // Accounts passed to the instruction
    let funding_account = &accounts[0]; // The account that will fund the new account.
    let new_account = &accounts[1]; // The new account that will be created.

    // Ensure the funding account and new account are signers
    if !funding_account.is_signer() || !new_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Creating the instruction instance
    let create_account_ix = CreateAccount {
        from: funding_account,
        to: new_account,
        lamports,
        space,
        owner: owner_account.key(),
    };

    let (_pda, bump) = try_find_program_address(&[owner_account.key().as_ref()], &crate::ID)
        .ok_or(ProgramError::InvalidSeeds)?;
    let pda_ref = &[bump]; // prevent temporary value being freed
    let seeds = seeds!(b"seed", owner_account.key().as_ref(), pda_ref);
    let signer = Signer::from(&seeds);

    // Invoking the instruction
    create_account_ix.invoke_signed(&[signer])?;

    Ok(())
}
