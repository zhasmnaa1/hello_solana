use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Only the counter authority can update this counter")]
    Unauthorized,

    #[msg("Counter has reached the maximum value")]
    CounterOverflow,

    #[msg("Name must be no longer than 32 bytes")]
    NameTooLong,

    #[msg("Bio must be no longer than 200 bytes")]
    BioTooLong,
}