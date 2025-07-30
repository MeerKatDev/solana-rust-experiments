use pinocchio::sysvars::Sysvar;
use pinocchio::{
    ProgramResult, account_info::AccountInfo, program_error::ProgramError, sysvars::rent::Rent,
};
use pinocchio_system::instructions::CreateAccount;

/// Processes the `CreateAccount` instruction.
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
    lamports: u64, // Number of lamports to transfer to the new account.
    space: u64,    // Number of bytes to allocate for the new account.
) -> ProgramResult {
    // Accounts passed to the instruction
    let owner_account = &accounts[0]; // The account that will fund the new account.
    let new_account = &accounts[1]; // The new account that will be created.

    // Ensure the funding account and new account are signers
    if !owner_account.is_signer() || !new_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let lamports = Rent::get()?
        .minimum_balance(space as usize)
        .checked_add(lamports)
        .unwrap();

    CreateAccount {
        from: owner_account,
        to: new_account,
        lamports,
        space,
        owner: &crate::ID,
    }
    .invoke()?;

    Ok(())
}
