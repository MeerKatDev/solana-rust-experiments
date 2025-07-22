use crate::error::ErrorCode;
use crate::{CandidateAccount, PollAccount};
use alloc::string::String;
use borsh::BorshSerialize;

use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use pinocchio_log::logger::Logger;

pub fn initialize_candidate(
    accounts: &[AccountInfo],
    _poll_id: u64,
    candidate_name: String,
) -> ProgramResult {
    let mut logger = Logger::<100>::default();

    let poll_account = &accounts[0]; // Writable, owned by program,
    let candidate_account = &accounts[1]; // Signer (payer)

    // check if they are writable
    if !poll_account.is_writable() {
        logger.append("Account is not writable");
        logger.log();
        return Err(ProgramError::InvalidAccountData);
    }

    if !candidate_account.is_writable() {
        logger.append("Account is not writable");
        logger.log();
        return Err(ProgramError::IncorrectAuthority);
    }

    if !candidate_account.is_signer() {
        logger.append("Candidate account is not the signer");
        logger.log();
        return Err(ProgramError::MissingRequiredSignature);
    }

    // editing candidate account

    // let mut candidate_data = candidate_account
    //     .try_borrow_mut_data()
    //     .map_err(|_| ProgramError::InvalidAccountData)?;
    // let mut candidate_account = CandidateAccount::try_from_slice(&candidate_data)
    //     .map_err(|_e| ProgramError::InvalidAccountData)?;

    if candidate_name.len() > 32 {
        return Err(ProgramError::Custom(ErrorCode::NameTooLong.into()));
    }

    let candidate_data = CandidateAccount {
        candidate_name,
        candidate_votes: 0,
    };

    // get memory "booked" by candidate account
    let mut data = candidate_account
        .try_borrow_mut_data()
        .map_err(|_| ProgramError::InvalidAccountData)?;

    candidate_data
        .serialize(&mut &mut data[..])
        .map_err(|_e| ProgramError::InvalidAccountData)?;

    // editing poll account
    let (mut poll, mut data) = PollAccount::load_mut(poll_account)?;
    poll.increment_option_index();
    poll.save(&mut data)?;

    Ok(())
}