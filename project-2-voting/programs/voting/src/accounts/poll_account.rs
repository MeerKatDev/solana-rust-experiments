use borsh::{BorshDeserialize, BorshSerialize};
use pinocchio::{
    ProgramResult,
    account_info::AccountInfo,
    program_error::ProgramError,
    account_info::RefMut // this or core?
};
use alloc::string::String;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PollAccount {
    pub poll_id: u64,
    pub poll_name: String,
    pub poll_description: String,
    pub poll_voting_start: u64,
    pub poll_voting_end: u64,
    pub poll_option_index: u64,
}

impl PollAccount {
    pub fn increment_option_index(&mut self) {
        self.poll_option_index += 1;
    }

    /// Loads a `PollAccount` from account data (expects valid data format).
    pub fn load_mut(account: &AccountInfo) -> Result<(Self, RefMut<[u8]>), ProgramError> {
        let data = account
            .try_borrow_mut_data()
            .map_err(|_| ProgramError::InvalidAccountData)?;

        let poll = PollAccount::try_from_slice(&data)
            .map_err(|_| ProgramError::InvalidAccountData)?;

        Ok((poll, data))
    }

    /// Save this instance back into the account data.
    pub fn save(&self, dst: &mut [u8]) -> ProgramResult {
        self.serialize(&mut &mut dst[..])
            .map_err(|_| ProgramError::InvalidAccountData)
    }
}