use anchor_lang::prelude::*;
use crate::state::fpl::*;
use crate::errors::*;

pub struct FplGlobalParams {
    pub current_gameweek: u8,
    pub season_start: i64,
    pub season_end: i64, 
    pub api_url: String,
}

//Fplinit

#[derive(Accounts)]
pub struct InitializeFplGlobal<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 1 + 8 + 8 + 100 + 1, // Adjust space calculation as needed
        seeds = [b"fpl-global"],
        bump
    )]
    pub global_state: Account<'info, FplGlobalState>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_fpl_global(ctx: Context<InitializeFplGlobal>, params: FplGlobalParams) -> Result<()> {
    let global_state = &mut ctx.accounts.global_state;
    let bump = *ctx.bumps.get("global_state").unwrap();
    
    global_state.admin = ctx.accounts.admin.key();
    global_state.current_gameweek = params.current_gameweek;
    global_state.season_start = params.season_start;
    global_state.season_end = params.season_end;
    global_state.api_url = params.api_url;
    global_state.bump = bump;
    
    Ok(())
}


//fpl_user_registration
