use anchor_lang::prelude::*;

#[event]
pub struct Payout {
    pub seller: Pubkey,
    pub claimed: u64,
    pub fees_taken: u64,
    pub timestamp: i64,
}
