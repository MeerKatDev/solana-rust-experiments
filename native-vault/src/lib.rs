#![allow(unexpected_cfgs)]

use pinocchio::{
    ProgramResult, account_info::AccountInfo, entrypoint, program_error::ProgramError,
};
// use pinocchio::pubkey::Pubkey;
use ixs::VaultInstructions;
use pinocchio::pubkey::Pubkey;
use pinocchio_pubkey::pubkey;

mod ixs;
use ixs::*;

// Always define program IDs as constants using `pubkey!`.
// This avoids runtime costs of deriving keys dynamically.
const ID: Pubkey = pubkey!("8Qe6bE7P6pHrxWYXnkaidUGfXQkN7bcZ7PkYzNQXnZJD");

// Macro that declares `process_instruction` as the program's entry point.
entrypoint!(process_instruction);

/// Main function to process instructions.
pub fn process_instruction(
    program_id: &Pubkey,      // Reference to the program ID.
    accounts: &[AccountInfo], // List of accounts involved in the transaction.
    data: &[u8],              // Serialized instruction data (byte array).
) -> ProgramResult {
    // Ensure the program ID matches the expected value. This prevents hijacking by another program.
    if program_id != &crate::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Parse the instruction discriminator and its associated data.
    // `split_first` separates the first byte (discriminator) from the rest (payload).
    let (discriminator, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    let lamports = u64::from_le_bytes([
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
    ]);

    match VaultInstructions::try_from(discriminator)? {
        VaultInstructions::Deposit => deposit::process(accounts, lamports),
        VaultInstructions::Withdraw => withdraw::process(accounts, lamports),
    }
}
