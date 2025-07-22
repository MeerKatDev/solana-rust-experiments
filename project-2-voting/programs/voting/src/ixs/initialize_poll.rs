use crate::error::ErrorCode;
use crate::PollAccount;
use alloc::string::String;
use borsh::BorshSerialize;

use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use pinocchio_log::logger::Logger;

pub fn initialize_poll(
    accounts: &[AccountInfo],
    poll_id: u64,
    poll_name: String,
    poll_description: String,
    poll_voting_start: u64,
    poll_voting_end: u64,
) -> ProgramResult {
    let mut logger = Logger::<100>::default();

    let poll_account = &accounts[0]; // Writable, owned by program
                                     // let initializer = &accounts[1]; // Signer, who initializes the poll, not necessarily the candidate

    // if !initializer.is_signer() {
    //     return Err(ProgramError::MissingRequiredSignature);
    // }

    // if !poll_account.is_owned_by(initializer.key()) {
    //     logger.append("Poll account is not owned by initializer");
    //     logger.log();

    //     return Err(ProgramError::IllegalOwner);
    // }

    if poll_description.len() > 280 {
        return Err(ProgramError::Custom(ErrorCode::DescriptionTooLong.into()));
    }

    // This is good for creation from zero
    let new_poll_account = PollAccount {
        poll_id,
        poll_name,
        poll_description,
        poll_voting_start,
        poll_voting_end,
        poll_option_index: 0,
    };

    let mut data = poll_account.try_borrow_mut_data().unwrap();

    new_poll_account
        .serialize(&mut &mut data[..])
        .map_err(|_e| ProgramError::InvalidAccountData)?;

    logger.append("Poll initialized successfully");
    logger.log();

    Ok(())
}
