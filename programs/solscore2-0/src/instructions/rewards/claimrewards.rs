use anchor_lang::prelude::*;
use crate::state::reward::*;
use crate::state::stake::*;
use crate::state::fpl::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"stake", user.key().as_ref(), &stake_id.to_le_bytes()],
        bump = stake.bump,
        constraint = stake.owner == user.key() @ ErrorCode::UnauthorizedAccess,
        constraint = stake.is_active @ ErrorCode::StakeNotActive,
    )]
    pub stake: Account<'info, Stake>,
    
    #[account(
        seeds = [b"fpl-user", user.key().as_ref()],
        bump = fpl_user.bump,
    )]
    pub fpl_user: Account<'info, FplUser>,
    
    #[account(
        seeds = [b"reward-config"],
        bump = reward_config.bump,
    )]
    pub reward_config: Account<'info, RewardConfig>,
    
    #[account(
        mut,
        seeds = [b"reward-pool"],
        bump = reward_pool.bump,
    )]
    pub reward_pool: Account<'info, RewardPool>,
    
    #[account(
        mut,
        seeds = [b"reward-vault"],
        bump,
    )]
    /// CHECK: This is the PDA that holds rewards
    pub reward_vault: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
    let stake = &mut ctx.accounts.stake;
    let fpl_user = &ctx.accounts.fpl_user;
    let reward_config = &ctx.accounts.reward_config;
    let reward_pool = &mut ctx.accounts.reward_pool;
    let reward_vault = &ctx.accounts.reward_vault;
    let user = &ctx.accounts.user;
    
    let current_time = Clock::get()?.unix_timestamp;
    let time_since_last_claim = current_time - stake.last_claim_time;
    
    // Check if enough time has passed for claiming
    if time_since_last_claim < reward_config.distribution_frequency as i64 {
        return Err(error!(ErrorCode::TooEarlyToClaim));
    }
    
    // Calculate rewards based on stake amount, time, and FPL performance
    let base_reward = calculate_base_reward(
        stake.amount,
        time_since_last_claim as u64,
        reward_config.base_apy,
    );
    
    // Performance multiplier based on FPL score
    let performance_factor = 100 + (fpl_user.weekly_score * reward_config.score_multiplier as u32) / 100;
    let total_reward = (base_reward * performance_factor as u64) / 100;
    
    // Check if reward pool has enough funds
    if total_reward > reward_pool.total_rewards - reward_pool.distributed_rewards {
        return Err(error!(ErrorCode::NoRewardsAvailable));
    }
    
    // Update reward pool
    reward_pool.distributed_rewards += total_reward;
    
    // Update stake last claim time
    stake.last_claim_time = current_time;
    
    // Transfer rewards to user
    let reward_vault_bump = *ctx.bumps.get("reward_vault").unwrap();
    let seeds = &[b"reward-vault".as_ref(), &[reward_vault_bump]];
    let signer = &[&seeds[..]];
    
    anchor_lang::solana_program::program::invoke_signed(
        &anchor_lang::solana_program::system_instruction::transfer(
            &reward_vault.key(),
            &user.key(),
            total_reward,
        ),
        &[
            reward_vault.to_account_info(),
            user.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        signer,
    )?;
    
    Ok(())
}

// Helper function to calculate base rewards
fn calculate_base_reward(stake_amount: u64, time_period: u64, apy: u8) -> u64 {
    // Convert APY to per-second rate
    // APY is in percentage points, so convert to decimal
    let apy_decimal = apy as f64 / 100.0;
    
    // Convert to per-second rate (compounding once per year)
    // (1 + r)^t = (1 + apy)^(t/31536000)
    // where 31536000 is seconds in a year
    let seconds_in_year = 31_536_000u64;
    let time_fraction = time_period as f64 / seconds_in_year as f64;
    
    // Calculate reward amount
    let reward_multiplier = (1.0 + apy_decimal).powf(time_fraction) - 1.0;
    let reward_amount = (stake_amount as f64 * reward_multiplier) as u64;
    
    reward_amount
}

// to be reworked in full
