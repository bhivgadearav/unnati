use crate::state::{GlobalInfo, SellerInfo};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
           init,
           seeds = [GLOBAL_INFO_STATIC_SEED],
           bump,
           payer = signer,
           space = 8 + GlobalInfo::LEN,
       )]
    pub global_info: Account<'info, GlobalInfo>,
}

#[derive(Accounts)]
pub struct InitializeSeller<'info> {
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
           init,
           seeds = [SELLER_INFO_STATIC_SEED, signer.key().as_ref()],
           bump,
           payer = signer,
           space = 8 + SellerInfo::LEN,
       )]
    pub seller_info: Account<'info, SellerInfo>,
}

#[derive(Accounts)]
pub struct Checkout<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
           mut,
           seeds = [GLOBAL_INFO_STATIC_SEED],
           bump,
           constraint = global_info.is_initialized
           @ ProgramError::UninitializedAccount
       )]
    pub global_info: Account<'info, GlobalInfo>,
    pub token_program: Program<'info, Token>,
    #[account(
            mut,
            constraint = currency_mint.is_initialized &&
            currency_mint.key() == global_info.currency
        )]
    pub currency_mint: Account<'info, Mint>,
    #[account(
            mut,
            seeds = [
                SELLER_INFO_STATIC_SEED,
                signer.key().as_ref()
            ],
            bump,
            constraint = seller_info.is_initialized &&
            seller_info.seller == signer.key()
        )]
    pub seller_info: Account<'info, SellerInfo>,
    #[account(
            mut,
            token::mint = currency_mint,
            token::authority = signer,
        )]
    pub signer_token_ata: Account<'info, TokenAccount>,
}
