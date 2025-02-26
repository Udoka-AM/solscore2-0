use anchor_lang::prelude::*;
use crate::state::fpl::*;
use crate::errors::*;


#[derive(Accounts)]
pub struct RegisterFplUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 50 + 200 + 4 + 4 + 8 + 1, // Adjust space calculation as needed
        seeds = [b"fpl-user", user.key().as_ref()],
        bump
    )]
    pub fpl_user: Account<'info, FplUser>,
    
    pub global_state: Account<'info, FplGlobalState>,
    pub system_program: Program<'info, System>,
}

pub fn register_fpl_user(ctx: Context<RegisterFplUser>, fpl_id: String) -> Result<()> {
    let fpl_user = &mut ctx.accounts.fpl_user;
    let bump = *ctx.bumps.get("fpl_user").unwrap();
    
    // Validate FPL ID format (could add more validation)
    if fpl_id.len() == 0 || fpl_id.len() > 20 {
        return Err(error!(ErrorCode::InvalidFplId));
    }
    
    fpl_user.authority = ctx.accounts.user.key();
    fpl_user.fpl_id = fpl_id;
    fpl_user.team_data = Vec::new(); // Will be populated later via API
    fpl_user.weekly_score = 0;
    fpl_user.total_score = 0;
    fpl_user.last_updated = Clock::get()?.unix_timestamp;
    fpl_user.bump = bump;
    
    Ok(())
}