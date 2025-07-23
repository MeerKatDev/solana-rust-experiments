use crate::alloc::string::ToString;
use alloc::string::String;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CandidateAccount {
    pub candidate_name: String,
    pub candidate_votes: u64,
}

impl Default for CandidateAccount {
    fn default() -> Self {
        Self {
            candidate_name: "n".repeat(Self::MAX_NAME_LENGTH),
            candidate_votes: 0,
        }
    }
}

impl CandidateAccount {
    pub const MAX_NAME_LENGTH: usize = 30;

    pub fn new(candidate_name: String) -> Self {
        Self {
            candidate_name: Self::checked_name(&candidate_name),
            candidate_votes: 0,
        }
    }

    pub fn checked_name(name: &str) -> String {
        Self::pad_string_null(name.to_string(), Self::MAX_NAME_LENGTH)
    }

    // duplicate
    fn pad_string_null(mut s: String, len: usize) -> String {
        if s.len() > len {
            s.truncate(len);
        } else {
            s.push_str(&"\0".repeat(len - s.len()));
        }
        s
    }
}
