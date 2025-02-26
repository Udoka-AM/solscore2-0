use anchor_lang::prelude::*;
use crate::state::stake::*;
use crate::state::treasury::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct Unstake<'info> {
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
        seeds = [b"stake-config"],
        bump = stake_config.bump,
    )]
    pub stake_config: Account<'info, StakeConfig>,
    
    #[account(
        mut,
        seeds = [b"stake-vault"],
        bump,
    )]
    /// CHECK: This is the PDA that holds the staked SOL
    pub stake_vault: UncheckedAccount<'info>,
    
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,
    
    pub system_program: Program<'info, System>,
}

pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
    let stake = &mut ctx.accounts.stake;
    let stake_config = &ctx.accounts.stake_config;
    let treasury = &mut ctx.accounts.treasury;
    let user = &ctx.accounts.user;
    let stake_vault = &ctx.accounts.stake_vault;
    
    let current_time = Clock::get()?.unix_timestamp;
    let stake_duration = current_time - stake.start_time;
    
    // Calculate amount to return after potential penalties
    let mut return_amount = stake.amount;
    
    // Apply early withdrawal fee if applicable
    if stake_duration < stake.lock_period as i64 {
        let fee_amount = (stake.amount * stake_config.early_withdrawal_fee as u64) / 100;
        return_amount -= fee_amount;
        
        // Add fee to treasury
        treasury.total_fees += fee_amount;
    }
    
    // Mark stake as inactive
    stake.is_active = false;
    
    // Transfer SOL back to the user
    let stake_vault_bump = *ctx.bumps.get("stake_vault").unwrap();
    let seeds = &[b"stake-vault".as_ref(), &[stake_vault_bump]];
    let signer = &[&seeds[..]];
    
    anchor_lang::solana_program::program::invoke_signed(
        &anchor_lang::solana_program::system_instruction::transfer(
            &stake_vault.key(),
            &user.key(),
            return_amount,
        ),
        &[
            stake_vault.to_account_info(),
            user.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        signer,
    )?;
    
    Ok(())
}