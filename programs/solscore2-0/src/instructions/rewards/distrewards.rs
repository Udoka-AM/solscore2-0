use anchor_lang::prelude::*;
use crate::state::reward::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"reward-pool"],
        bump = reward_pool.bump,
        constraint = reward_pool.admin == admin.key() @ ErrorCode::UnauthorizedAccess,
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

pub fn distribute_rewards(ctx: Context<DistributeRewards>, amount: u64) -> Result<()> {
    let reward_pool = &mut ctx.accounts.reward_pool;
    let admin = &ctx.accounts.admin;
    let reward_vault = &ctx.accounts.reward_vault;
    
    // Transfer SOL to the reward vault
    let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(
        &admin.key(),
        &reward_vault.key(),
        amount,
    );
    
    anchor_lang::solana_program::program::invoke(
        &transfer_instruction,
        &[
            admin.to_account_info(),
            reward_vault.to_account_info(),
        ],
    )?;
    
    // Update reward pool
    reward_pool.total_rewards += amount;
    
    Ok(())
}

// to be reworked in full
