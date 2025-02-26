use anchor_lang::prelude::*;
use crate::state::treasury::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct WithdrawTreasury<'info> {
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
    
    #[account(
        mut,
        seeds = [b"treasury-vault"],
        bump,
    )]
    /// CHECK: This is the PDA that holds treasury funds
    pub treasury_vault: UncheckedAccount<'info>,
    
    #[account(mut)]
    /// CHECK: This is where withdrawn funds will be sent
    pub recipient: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn withdraw_treasury(ctx: Context<WithdrawTreasury>, amount: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let treasury_vault = &ctx.accounts.treasury_vault;
    let recipient = &ctx.accounts.recipient;
    
    // Get treasury vault balance
    let vault_balance = ctx.accounts.treasury_vault.lamports();
    
    // Calculate maximum withdrawable amount based on reserve percentage
    let reserved_amount = (vault_balance * treasury.reserve_percentage as u64) / 100;
    let max_withdrawable = vault_balance.saturating_sub(reserved_amount);
    
    // Validate withdrawal amount
    if amount > max_withdrawable {
        return Err(error!(ErrorCode::ExceedsWithdrawalLimit));
    }
    
    // Transfer SOL from treasury vault to recipient
    let treasury_vault_bump = *ctx.bumps.get("treasury_vault").unwrap();
    let seeds = &[b"treasury-vault".as_ref(), &[treasury_vault_bump]];
    let signer = &[&seeds[..]];
    
    anchor_lang::solana_program::program::invoke_signed(
        &anchor_lang::solana_program::system_instruction::transfer(
            &treasury_vault.key(),
            &recipient.key(),
            amount,
        ),
        &[
            treasury_vault.to_account_info(),
            recipient.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        signer,
    )?;
    
    Ok(())
}