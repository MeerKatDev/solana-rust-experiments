#![allow(unexpected_cfgs)]
#![no_std]

extern crate alloc;
use borsh::BorshDeserialize;
use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

pub mod accounts;
pub mod error;
pub mod ixs;

use accounts::{CandidateAccount, PollAccount};
use ixs::VotingInstruction;

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
            ixs::initialize_poll(accounts, poll_id, name, description, start_time, end_time)?;
        }
        VotingInstruction::InitializeCandidate { poll_id, candidate } => {
            ixs::initialize_candidate(accounts, poll_id, candidate)?
        }
        VotingInstruction::Vote { poll_id, candidate } => ixs::vote(accounts, poll_id, candidate)?,
    }

    Ok(())
}
