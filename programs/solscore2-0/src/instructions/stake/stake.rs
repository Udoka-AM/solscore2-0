use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};
use crate::state::stake::*;
use crate::state::fpl::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct CreateStake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"fpl-user", user.key().as_ref()],
        bump = fpl_user.bump,
    )]
    pub fpl_user: Account<'info, FplUser>,
    
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 8 + 8 + 32 + 1 + 8 + 1, // Adjust space calculation
        seeds = [b"stake", user.key().as_ref(), &stake_count.to_le_bytes()],
        bump
    )]
    pub stake: Account<'info, Stake>,
    
    #[account(
        seeds = [b"stake-config"],
        bump = stake_config.bump,
    )]
    pub stake_config: Account<'info, StakeConfig>,
    
    #[account(
        mut,
        seeds = [b"stake-vault"],
        bump,
    )]
    /// CHECK: This is the PDA that will receive the staked SOL
    pub stake_vault: UncheckedAccount<'info>,
    
    #[account(
        mut,
        seeds = [b"stake-count", user.key().as_ref()],
        bump,
    )]
    pub stake_count: Account<'info, StakeCount>,
    
    pub system_program: Program<'info, System>,
}

pub fn create_stake(ctx: Context<CreateStake>, amount: u64, lock_period: u64) -> Result<()> {
    let user = &ctx.accounts.user;
    let stake = &mut ctx.accounts.stake;
    let stake_config = &ctx.accounts.stake_config;
    let stake_vault = &ctx.accounts.stake_vault;
    let stake_count = &mut ctx.accounts.stake_count;
    let bump = *ctx.bumps.get("stake").unwrap();
    
    // Validate stake amount
    if amount < stake_config.min_stake_amount || amount > stake_config.max_stake_amount {
        return Err(error!(ErrorCode::InvalidStakeAmount));
    }
    
    // Validate lock period
    if !stake_config.lock_options.contains(&lock_period) {
        return Err(error!(ErrorCode::InvalidLockPeriod));
    }
    
    // Transfer SOL to the stake vault
    let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(
        &user.key(),
        &stake_vault.key(),
        amount,
    );
    
    anchor_lang::solana_program::program::invoke(
        &transfer_instruction,
        &[
            user.to_account_info(),
            stake_vault.to_account_info(),
        ],
    )?;
    
    // Initialize stake account
    stake.owner = user.key();
    stake.amount = amount;
    stake.start_time = Clock::get()?.unix_timestamp;
    stake.lock_period = lock_period;
    stake.fpl_user = ctx.accounts.fpl_user.key();
    stake.is_active = true;
    stake.last_claim_time = Clock::get()?.unix_timestamp;
    stake.bump = bump;
    
    // Increment stake count
    stake_count.count += 1;
    
    Ok(())
}