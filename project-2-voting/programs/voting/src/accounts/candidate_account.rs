use borsh::{BorshDeserialize, BorshSerialize};
use alloc::string::String;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CandidateAccount {
    pub candidate_name: String,
    pub candidate_votes: u64,
}