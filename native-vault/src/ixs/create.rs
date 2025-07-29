use pinocchio::{
    ProgramResult, account_info::AccountInfo, instruction::Signer, program_error::ProgramError,
    pubkey::try_find_program_address, seeds,
};
use pinocchio::{msg, program};
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

    let owner = owner_account.key();

    // Creating the instruction instance
    let create_account_ix = CreateAccount {
        from: owner_account,
        to: new_account,
        lamports,
        space,
        owner: owner,
    };


    // let (pda, bump) = try_find_program_address(&[owner.as_ref()], &crate::ID)
    //     .ok_or(ProgramError::InvalidSeeds)?;
    // let pda_ref = &[bump]; // prevent temporary value being freed
    msg!(&format!("owner_account: {:?}", owner_account.key()).to_string());
    msg!(&format!("new_account: {:?}", new_account.key()).to_string());
    // let seeds = seeds!(b"seed", owner.as_ref(), pda_ref);
    // let signer = Signer::from(&seeds);
    // msg!(&format!("signer: {:?}", &signer).to_string());

    msg!("Invoking instruction");
    // Invoking the instruction
    // create_account_ix.invoke_signed(&[signer])?;
    create_account_ix.invoke()?;
    // program::invoke_signed,
    msg!("Invoking signed");

    Ok(())
}
