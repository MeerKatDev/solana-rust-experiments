use crate::error::ErrorCode;
use crate::{CandidateAccount, PollAccount};
use alloc::string::String;
use borsh::{BorshSerialize, BorshDeserialize};

use pinocchio::{
	ProgramResult,
	account_info::AccountInfo, 
	sysvars::{clock::Clock, Sysvar}, 
	program_error::ProgramError, 
};

pub fn vote(accounts: &[AccountInfo], _poll_id: u64, _candidate_name: String) -> ProgramResult {
    let poll_account = &accounts[0]; // Writable, owned by program,
    let candidate_account = &accounts[1]; // Signer (payer)

    // this has to be read-only
    let poll_data = poll_account
        .try_borrow_data()
        .map_err(|_| ProgramError::InvalidAccountData)?;
    let poll_account =
        PollAccount::try_from_slice(&poll_data).map_err(|_e| ProgramError::InvalidAccountData)?;

    // checking time boundaries for voting

    let current_time = Clock::get()?.unix_timestamp;

    if current_time > (poll_account.poll_voting_end as i64) {
        return Err(ProgramError::Custom(ErrorCode::VotingEnded.into()));
    }

    if current_time <= (poll_account.poll_voting_start as i64) {
        return Err(ProgramError::Custom(ErrorCode::VotingNotStarted.into()));
    }

    // editing candidate account

    // this has to be mutable / modifiable
    let mut candidate_data = candidate_account
        .try_borrow_mut_data()
        .map_err(|_| ProgramError::InvalidAccountData)?;
    let mut candidate_account = CandidateAccount::try_from_slice(&candidate_data)
        .map_err(|_e| ProgramError::InvalidAccountData)?;

    candidate_account.candidate_votes += 1;

    candidate_account
        .serialize(&mut &mut candidate_data[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    Ok(())
}
