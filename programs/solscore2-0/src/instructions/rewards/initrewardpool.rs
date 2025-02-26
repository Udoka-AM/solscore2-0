use anchor_lang::prelude::*;
use crate::state::reward::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct InitializeRewardPool<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(
        seeds = [b"reward-config"],
        bump = reward_config.bump,
        constraint = reward_config.admin == admin.key() @ ErrorCode::UnauthorizedAccess,
    )]
    pub reward_config: Account<'info, RewardConfig>,
    
    #[account(
        init,
        payer = admin,
        space = 8 + 8 + 8 + 32 + 1, // Adjust space calculation
        seeds = [b"reward-pool"],
        bump
    )]
    pub reward_pool: Account<'info, RewardPool>,
    
    #[account(
        init,
        payer = admin,
        space = 8, // Minimal space for token vault
        seeds = [b"reward-vault"],
        bump
    )]
    /// CHECK: This is just a PDA that will receive SOL
    pub reward_vault: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_reward_pool(ctx: Context<InitializeRewardPool>) -> Result<()> {
    let reward_pool = &mut ctx.accounts.reward_pool;
    let bump = *ctx.bumps.get("reward_pool").unwrap();
    
    reward_pool.total_rewards = 0;
    reward_pool.distributed_rewards = 0;
    reward_pool.admin = ctx.accounts.admin.key();
    reward_pool.bump = bump;
    
    Ok(())
}