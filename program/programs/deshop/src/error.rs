use anchor_lang::prelude::*;

#[error_code]
pub enum UnnatiErrorCodes {
    #[msg("Program already initialized.")]
    ProgramInitialized,
    #[msg("Amount value sent is zero.")]
    ZeroAmountValue,
    #[msg("Provided pubkey cannot be default / zero")]
    InvalidPubkey,
    #[msg("Fee >= amount (insufficient after fee)")]
    AmountInsufficientAfterFee,
    #[msg("Overflow")]
    Overflow,
}
