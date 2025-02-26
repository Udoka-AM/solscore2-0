use anchor_lang::prelude::*;
use crate::state::treasury::*;
use crate::errors::*;

pub struct TreasuryParams {
    pub protocol_fee: u8,
    pub reserve_percentage: u8,
}

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 8 + 1 + 1 + 1, // Adjust space calculation
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(
        init,
        payer = admin,
        space = 8, // Minimal space for treasury vault
        seeds = [b"treasury-vault"],
        bump
    )]
    /// CHECK: This is just a PDA that will receive SOL
    pub treasury_vault: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_treasury(ctx: Context<InitializeTreasury>, params: TreasuryParams) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let bump = *ctx.bumps.get("treasury").unwrap();
    
    // Validate parameters
    if params.protocol_fee > 100 || params.reserve_percentage > 100 {
        return Err(error!(ErrorCode::InvalidTreasuryParameter));
    }
    
    treasury.admin = ctx.accounts.admin.key();
    treasury.total_fees = 0;
    treasury.protocol_fee = params.protocol_fee;
    treasury.reserve_percentage = params.reserve_percentage;
    treasury.bump = bump;
    
    Ok(())
}