use pinocchio::pubkey::try_find_program_address;
use pinocchio::{ProgramResult, account_info::AccountInfo, program_error::ProgramError};
use pinocchio_system::instructions::Transfer;

/// Processes the deposit instruction.
pub fn process(accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    // Validate the account array structure.
    let [signer, vault, _system_program] = accounts else {
        return Err(ProgramError::InvalidAccountData);
    };

    // Derive the Program Derived Address (PDA) for the vault and validate it.
    let (pda, _bump) = try_find_program_address(&[signer.key().as_ref()], &crate::ID)
        .ok_or(ProgramError::InvalidSeeds)?;

    assert_eq!(&pda, vault.key()); // Ensure the PDA matches the vault's public key.

    // Perform the transfer of lamports from the signer to the vault account.

    let transfer_ix = Transfer {
        from: signer,
        to: vault,
        lamports,
    };
    transfer_ix.invoke()?;

    Ok(())
    // invoke(
    //     &transfer(signer.key, vault.key, lamports),
    //     accounts, // Pass account references required for the transfer.
    // )
}
