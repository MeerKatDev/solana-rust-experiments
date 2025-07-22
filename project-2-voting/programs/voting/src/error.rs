#[repr(u32)]
pub enum ErrorCode {
    VotingNotStarted,
    VotingEnded,
    NameTooLong,
    DescriptionTooLong,
}

impl From<ErrorCode> for u32 {
    fn from(err: ErrorCode) -> u32 {
        err as u32
    }
}
