use alloc::string::String;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CandidateAccount {
    pub candidate_name: String,
    pub candidate_votes: u64,
}
