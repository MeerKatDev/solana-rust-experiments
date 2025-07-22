use alloc::string::String;
use alloc::vec;
use borsh::{BorshDeserialize, BorshSerialize};
use core::mem;
use pinocchio::{
    account_info::AccountInfo,
    account_info::RefMut, // this or core?
    program_error::ProgramError,
    ProgramResult,
};

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
            poll_name: "a".repeat(Self::MAX_NAME_LENGTH),
            poll_description: String::from_utf8(vec![0u8; Self::MAX_DESCRIPTION_LENGTH]).unwrap(),
            poll_voting_start: 0,
            poll_voting_end: 0,
            poll_option_index: 0,
        }
    }
}

impl PollAccount {
    pub const MAX_NAME_LENGTH: usize = 50;
    pub const MAX_DESCRIPTION_LENGTH: usize = 280;
    pub const SIZE: usize =
        mem::size_of::<Self>() + Self::MAX_NAME_LENGTH + Self::MAX_DESCRIPTION_LENGTH;

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
}
