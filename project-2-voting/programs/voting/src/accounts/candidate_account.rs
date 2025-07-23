use crate::alloc::string::ToString;
use alloc::string::String;
use borsh::{BorshDeserialize, BorshSerialize};
use pinocchio::{
    account_info::AccountInfo, account_info::RefMut, program_error::ProgramError, ProgramResult,
};

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
    pub const MAX_NAME_LENGTH: usize = 16;

    pub fn new(candidate_name: String) -> Self {
        Self {
            candidate_name: Self::checked_name(&candidate_name),
            candidate_votes: 0,
        }
    }

    pub fn increment_candidate_votes(&mut self) {
        self.candidate_votes += 1;
    }

    /// Loads a `CandidateAccount` from account data (expects valid data format).
    pub fn load_mut(account: &AccountInfo) -> Result<(Self, RefMut<[u8]>), ProgramError> {
        let data = account
            .try_borrow_mut_data()
            .map_err(|_| ProgramError::InvalidAccountData)?;

        let poll = Self::try_from_slice(&data).map_err(|_| ProgramError::InvalidAccountData)?;

        Ok((poll, data))
    }

    /// Save this instance back into the account data.
    pub fn save(&self, dst: &mut [u8]) -> ProgramResult {
        self.serialize(&mut &mut dst[..])
            .map_err(|_| ProgramError::InvalidAccountData)
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
