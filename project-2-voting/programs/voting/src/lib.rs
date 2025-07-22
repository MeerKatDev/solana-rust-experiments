#![allow(unexpected_cfgs)]
// #![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec;

use borsh::{BorshDeserialize, BorshSerialize};
use pinocchio::{
    account_info::AccountInfo,
    entrypoint, msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};
use pinocchio_log::logger::Logger;

pub mod accounts;
use accounts::PollAccount;

#[repr(u32)]
pub enum ErrorCode {
    VotingNotStarted,
    VotingEnded,
    NameTooLong,
    DescriptionTooLong,
}

impl From<ErrorCode> for u32 {
    fn from(err: ErrorCode) -> u32 {
        err as u32
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CandidateAccount {
    pub candidate_name: String,
    pub candidate_votes: u64,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VotingInstruction {
    InitializePoll {
        poll_id: u64,
        name: String,
        description: String,
        start_time: u64,
        end_time: u64,
    },
    InitializeCandidate {
        poll_id: u64,
        candidate: String,
    },
    Vote {
        poll_id: u64,
        candidate: String,
    },
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    ix_data: &[u8],
) -> ProgramResult {
    let instruction = VotingInstruction::try_from_slice(ix_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        VotingInstruction::InitializePoll {
            poll_id,
            name,
            description,
            start_time,
            end_time,
        } => {
            process_initialize_poll(accounts, poll_id, name, description, start_time, end_time)?;
        }
        VotingInstruction::InitializeCandidate { poll_id, candidate } => {
            process_initialize_candidate(accounts, poll_id, candidate)?
        }
        VotingInstruction::Vote { poll_id, candidate } => {
            process_vote(accounts, poll_id, candidate)?
        }
    }

    Ok(())
}

fn process_initialize_candidate(
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

fn process_vote(accounts: &[AccountInfo], _poll_id: u64, _candidate_name: String) -> ProgramResult {
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

fn process_initialize_poll(
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
    // let new_poll_account = PollAccount {
    //     poll_id,
    //     poll_name,
    //     poll_description,
    //     poll_voting_start,
    //     poll_voting_end,
    //     poll_option_index: 0,
    // };

    let mut data = poll_account.try_borrow_mut_data().unwrap();

    let mut poll_account =
        PollAccount::try_from_slice(&data).map_err(|_e| ProgramError::InvalidAccountData)?;

    // NOTE: this assumes that the account is already present,
    // not that it has to be created.
    poll_account.poll_id = poll_id;
    poll_account.poll_name = poll_name;
    poll_account.poll_description = poll_description;
    poll_account.poll_voting_start = poll_voting_start;
    poll_account.poll_voting_end = poll_voting_end;
    poll_account.poll_option_index = 0;

    poll_account
        .serialize(&mut &mut data[..])
        .map_err(|_e| ProgramError::InvalidAccountData)?;

    logger.append("Poll initialized successfully");
    logger.log();

    Ok(())
}
