use alloc::string::String;
use borsh::{BorshDeserialize, BorshSerialize};
use pinocchio::{
    account_info::AccountInfo, account_info::RefMut, program_error::ProgramError, ProgramResult,
};
// test-only
use alloc::string::ToString;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PollAccount {
    pub poll_id: u64,
    pub poll_name: String,
    pub poll_description: String,
    pub poll_voting_start: u64,
    pub poll_voting_end: u64,
    pub poll_option_index: u64,
}

impl Default for PollAccount {
    fn default() -> Self {
        Self {
            poll_id: 0,
            poll_name: "n".repeat(Self::MAX_NAME_LENGTH),
            poll_description: "d".repeat(Self::MAX_DESCRIPTION_LENGTH),
            poll_voting_start: 0,
            poll_voting_end: 0,
            poll_option_index: 0,
        }
    }
}

impl PollAccount {
    pub const MAX_NAME_LENGTH: usize = 50;
    pub const MAX_DESCRIPTION_LENGTH: usize = 280;

    pub fn increment_option_index(&mut self) {
        self.poll_option_index += 1;
    }

    /// Loads a `PollAccount` from account data (expects valid data format).
    pub fn load_mut(account: &AccountInfo) -> Result<(Self, RefMut<[u8]>), ProgramError> {
        let data = account
            .try_borrow_mut_data()
            .map_err(|_| ProgramError::InvalidAccountData)?;

        let poll =
            PollAccount::try_from_slice(&data).map_err(|_| ProgramError::InvalidAccountData)?;

        Ok((poll, data))
    }

    /// Save this instance back into the account data.
    pub fn save(&self, dst: &mut [u8]) -> ProgramResult {
        self.serialize(&mut &mut dst[..])
            .map_err(|_| ProgramError::InvalidAccountData)
    }

    /// Used in tests for the moment
    /// it keeps the string length amount constant
    pub fn checked_name(name: &str) -> String {
        Self::pad_string_null(name.to_string(), Self::MAX_NAME_LENGTH)
    }

    pub fn checked_desc(desc: &str) -> String {
        Self::pad_string_null(desc.to_string(), Self::MAX_DESCRIPTION_LENGTH)
    }

    fn pad_string_null(mut s: String, len: usize) -> String {
        if s.len() > len {
            s.truncate(len);
        } else {
            s.push_str(&"\0".repeat(len - s.len()));
        }
        s
    }
}
