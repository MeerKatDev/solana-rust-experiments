use pinocchio::pubkey::try_find_program_address;
use pinocchio::{
    ProgramResult, account_info::AccountInfo, instruction::Signer, program_error::ProgramError,
    seeds,
};
use pinocchio_system::instructions::Transfer;

/// Processes the withdraw instruction.
pub fn process(accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    // Validate the account array structure.
    let [vault, signer, _system_program] = accounts else {
        return Err(ProgramError::InvalidAccountData);
    };

    // Derive the Program Derived Address (PDA) for the vault and validate it.
    let (pda, bump) = try_find_program_address(&[signer.key().as_ref()], &crate::ID)
        .ok_or(ProgramError::InvalidSeeds)?;

    assert_eq!(&pda, vault.key()); // Ensure the PDA matches the vault's public key.

    // Perform the transfer of lamports from the vault back to the signer account.
    let transfer_ix = Transfer {
        from: signer,
        to: vault,
        lamports,
    };

    let pda_ref = &[bump]; // prevent temporary value being freed
    let seeds = seeds!(b"seed", signer.key().as_ref(), pda_ref);
    let signer = Signer::from(&seeds);
    let signers = [signer];
    transfer_ix.invoke_signed(&signers)?;

    // transfer_ix.invoke_signed(accounts);

    Ok(())
    // invoke_signed(
    //     &transfer(vault.key, signer.key, lamports),
    //     accounts, // Pass account references required for the transfer.
    //     &[&[signer.key.as_ref(), &[bump]]], // Include PDA seeds for signing.
    // )
}
