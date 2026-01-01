use anchor_lang::prelude::*;
mod error;
mod event;
mod state;
mod instruction;

declare_id!("22XZU4FA95TFfgEgmASYutYwubKZvqaz94YF6qqNkJ3s");

#[program]
pub mod deshop {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instruction::handler::handle_initialize(ctx)
    }

    pub fn initialize_seller(ctx: Context<InitializeSeller>) -> Result<()> {
        instruction::handler::handle_initialize_seller(ctx)
    }

    pub fn checkout(ctx: Context<Checkout>) -> Result<()> {
        instruction::handler::handle_checkout(ctx)
    }
}
