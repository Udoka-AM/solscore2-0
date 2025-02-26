use anchor_lang::prelude::*;

#[account]
pub struct Treasury {
    pub admin: Pubkey,           // Admin authority
    pub total_fees: u64,         // Total collected fees
    pub protocol_fee: u8,        // Protocol fee percentage (0-100)
    pub reserve_percentage: u8,  // Percentage to keep as reserves
    pub bump: u8,                // PDA bump
}