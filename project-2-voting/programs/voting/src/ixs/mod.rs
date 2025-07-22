pub mod initialize_candidate;
pub use initialize_candidate::initialize_candidate;

pub mod initialize_poll;
pub use initialize_poll::initialize_poll;

pub mod vote;
pub use vote::vote;

use alloc::string::String;
use borsh::{BorshDeserialize, BorshSerialize};

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
