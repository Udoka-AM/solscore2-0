use anchor_lang::prelude::*;
use crate::state::reward::*;
use crate::errors::*;

pub struct RewardConfigParams {
    pub base_apy: u8,
    pub score_multiplier: u8,
    pub distribution_frequency: u64,
}

#[derive(Accounts)]
pub struct InitializeRewardConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 1 + 1 + 8 + 1, // Adjust space calculation
        seeds = [b"reward-config"],
        bump
    )]
    pub reward_config: Account<'info, RewardConfig>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_reward_config(ctx: Context<InitializeRewardConfig>, params: RewardConfigParams) -> Result<()> {
    let reward_config = &mut ctx.accounts.reward_config;
    let bump = *ctx.bumps.get("reward_config").unwrap();
    
    // Validate parameters
    if params.base_apy > 100 {
        return Err(error!(ErrorCode::InvalidRewardParameter));
    }
    
    reward_config.admin = ctx.accounts.admin.key();
    reward_config.base_apy = params.base_apy;
    reward_config.score_multiplier = params.score_multiplier;
    reward_config.distribution_frequency = params.distribution_frequency;
    reward_config.bump = bump;
    
    Ok(())
}