use anchor_lang::prelude::*;
use crate::state::treasury::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct UpdateTreasuryConfig<'info> {
    #[account(
        mut,
        constraint = treasury.admin == admin.key() @ ErrorCode::UnauthorizedAccess,
    )]
    pub admin: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,
}

pub fn update_treasury_config(
    ctx: Context<UpdateTreasuryConfig>, 
    new_protocol_fee: Option<u8>,
    new_reserve_percentage: Option<u8>,
) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    
    // Update protocol fee if provided
    if let Some(fee) = new_protocol_fee {
        if fee > 100 {
            return Err(error!(ErrorCode::InvalidTreasuryParameter));
        }
        treasury.protocol_fee = fee;
    }
    
    // Update reserve percentage if provided
    if let Some(reserve) = new_reserve_percentage {
        if reserve > 100 {
            return Err(error!(ErrorCode::InvalidTreasuryParameter));
        }
        treasury.reserve_percentage = reserve;
    }
    
    Ok(())
}