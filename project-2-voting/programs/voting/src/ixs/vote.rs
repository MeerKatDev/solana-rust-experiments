use crate::accounts::{CandidateAccount, PollAccount};
use crate::error::ErrorCode;
use alloc::string::String;
use borsh::BorshDeserialize;
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};

pub fn vote(accounts: &[AccountInfo], _poll_id: u64, _candidate_name: String) -> ProgramResult {
    // poll account may be read-only
    let poll_account = &accounts[0];
    // this has to be writable and mutable
    let candidate_account = &accounts[1]; // Signer (payer)

    if let Ok(poll_account) = unserialize_account(poll_account) {
        check_time_bounds(poll_account)?;
    }

    let (mut poll, mut data) = CandidateAccount::load_mut(candidate_account)?;
    poll.increment_candidate_votes();
    poll.save(&mut data)?;

    Ok(())
}

fn unserialize_account(poll_account: &AccountInfo) -> Result<PollAccount, ProgramError> {
    let poll_data = poll_account
        .try_borrow_data()
        .map_err(|_| ProgramError::InvalidAccountData)?;

    PollAccount::try_from_slice(&poll_data).map_err(|_e| ProgramError::InvalidAccountData)
}

fn check_time_bounds(poll_account: PollAccount) -> ProgramResult {
    let current_time = Clock::get()?.unix_timestamp;

    if current_time > (poll_account.poll_voting_end as i64) {
        return Err(ProgramError::Custom(ErrorCode::VotingEnded.into()));
    }

    if current_time <= (poll_account.poll_voting_start as i64) {
        return Err(ProgramError::Custom(ErrorCode::VotingNotStarted.into()));
    }

    Ok(())
}
