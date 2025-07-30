use pinocchio::pubkey::find_program_address;
use pinocchio::{
    ProgramResult, account_info::AccountInfo, instruction::Signer, program_error::ProgramError,
    seeds,
};
use pinocchio_system::instructions::Transfer;

pub const SEED: &[u8] = b"withdraw_seed";

/// Processes the withdraw instruction.
/// ### Accounts:
/// 0. `[WRITE, SIGNER]` The vault PDA account.
/// 1. `[WRITE, SIGNER]` The signer owning the funds in the vault account.
/// 2. `[NON-WRITE, NON-SIGNER]` System Account
pub fn process(accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    // Validate the account array structure.
    let [signer, vault, _system_program] = accounts else {
        return Err(ProgramError::InvalidAccountData);
    };

    if vault.is_signer() {
        // Wrong Signer
        return Err(ProgramError::Custom(28));
    }

    // the account is owned by system program
    // as long as it's not created onchain
    // if !vault.is_owned_by(&crate::ID) {
    //     return Err(ProgramError::InvalidAccountOwner);
    // }

    if !signer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Derive the Program Derived Address (PDA) for the vault and validate it.
    let seeds = &[SEED, signer.key().as_ref()];

    let (pda, bump) = find_program_address(seeds, &crate::ID);

    // Ensure the PDA matches the vault's public key.
    if pda.ne(vault.key()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    let transfer_ix = Transfer {
        from: vault,
        to: signer,
        lamports,
    };

    let pda_ref = [bump]; // prevent temporary value being freed
    let seeds = seeds!(SEED, signer.key().as_ref(), &pda_ref);
    let pda_signer = Signer::from(&seeds);
    let signers = [pda_signer];

    // Perform the transfer of lamports from the vault back to the signer account.
    transfer_ix.invoke_signed(&signers)?;

    Ok(())
}
