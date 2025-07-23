use crate::accounts::{CandidateAccount, PollAccount};
use crate::error::ErrorCode;
use alloc::string::String;
use borsh::BorshSerialize;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::logger::Logger;

pub fn initialize_candidate(
    accounts: &[AccountInfo],
    _poll_id: u64,
    candidate_name: String,
) -> ProgramResult {
    // check_accounts(accounts)?;

    let candidate_account = &accounts[0]; // Signer (payer)
    let poll_account = &accounts[1]; // Writable, owned by program,

    // creating candidate account

    if candidate_name.len() > CandidateAccount::MAX_NAME_LENGTH {
        return Err(ProgramError::Custom(ErrorCode::NameTooLong.into()));
    }

    let candidate_data = CandidateAccount::new(candidate_name);

    // get memory "booked" by candidate account
    let mut data = candidate_account.try_borrow_mut_data().unwrap();
    candidate_data.serialize(&mut &mut data[..]).unwrap();

    // editing poll account
    let (mut poll, mut data) = PollAccount::load_mut(poll_account)?;
    poll.increment_option_index();
    poll.save(&mut data)?;

    Ok(())
}

// TODO later will active this
#[allow(dead_code)]
fn check_accounts(accounts: &[AccountInfo]) -> ProgramResult {
    let mut logger = Logger::<100>::default();
    // TODO: remember that the signer is first
    let candidate_account = &accounts[0]; // Signer (payer)
    let poll_account = &accounts[1]; // Writable, owned by program,

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

    Ok(())
}
