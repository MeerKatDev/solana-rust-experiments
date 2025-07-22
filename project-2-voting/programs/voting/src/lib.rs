#![allow(unexpected_cfgs)]
#![no_std]

extern crate alloc;
use alloc::string::String;

use borsh::{BorshDeserialize, BorshSerialize};
use error::ErrorCode;
use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};
use pinocchio_log::logger::Logger;

mod error;

pub mod ixs;
use ixs::initialize_poll;

pub mod accounts;
use accounts::PollAccount;

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
            initialize_poll(accounts, poll_id, name, description, start_time, end_time)?;
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
