use pinocchio::pubkey::find_program_address;
use pinocchio::{ProgramResult, account_info::AccountInfo, program_error::ProgramError};
use pinocchio_system::instructions::Transfer;

pub const SEED: &[u8] = b"deposit_seed";

/// Processes the deposit instruction.
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The signer owning the funds in the vault account.
/// 1. `[WRITE, SIGNER]` The vault PDA account.
/// 2. `[NON-WRITE, NON-SIGNER]` System Account
pub fn process(accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    // Validate the account array structure.
    let [signer, vault, _system_program] = accounts else {
        return Err(ProgramError::InvalidAccountData);
    };

    // Derive the Program Derived Address (PDA) for the vault and validate it.
    let seeds = &[SEED, signer.key().as_ref()];

    let (pda, _bump) = find_program_address(seeds, &crate::ID);

    // Ensure the PDA matches the vault's public key.
    if pda.ne(vault.key()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // Perform the transfer of lamports from the signer to the vault account.

    let transfer_ix = Transfer {
        from: signer,
        to: vault,
        lamports,
    };

    // User signs the transfer; no PDA signing needed.
    transfer_ix.invoke()?;

    Ok(())
}
