use anchor_lang::prelude::*;

pub fn handle_initialize(ctx: Context<Initialize>) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}


pub fn handle_initialize_seller(ctx: Context<InitializeSeller>) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}

pub fn handle_checkout(ctx: Context<Checkout>) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}
