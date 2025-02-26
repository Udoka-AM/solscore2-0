use anchor_lang::prelude::*;
use crate::state::treasury::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct DepositToTreasury<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(
        mut,
        seeds = [b"treasury-vault"],
        bump,
    )]
    /// CHECK: This is the PDA that holds treasury funds
    pub treasury_vault: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn deposit_to_treasury(ctx: Context<DepositToTreasury>, amount: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let depositor = &ctx.accounts.depositor;
    let treasury_vault = &ctx.accounts.treasury_vault;
    
    // Validate amount
    if amount == 0 {
        return Err(error!(ErrorCode::InvalidDepositAmount));
    }
    
    // Transfer SOL to the treasury vault
    let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(
        &depositor.key(),
        &treasury_vault.key(),
        amount,
    );
    
    anchor_lang::solana_program::program::invoke(
        &transfer_instruction,
        &[
            depositor.to_account_info(),
            treasury_vault.to_account_info(),
        ],
    )?;
    
    // Update treasury
    treasury.total_fees += amount;
    
    Ok(())
}