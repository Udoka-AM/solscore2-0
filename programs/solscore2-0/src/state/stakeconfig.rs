use anchor_lang::prelude::*;

#[account]
pub struct Stake {
    pub owner: Pubkey,           // Stake owner
    pub amount: u64,             // Staked amount in SOL (lamports)
    pub start_time: i64,         // Start timestamp
    pub lock_period: u64,        // Lock period in seconds
    pub fpl_user: Pubkey,        // Associated FPL user account
    pub is_active: bool,         // Whether stake is active
    pub last_claim_time: i64,    // Last reward claim timestamp
    pub bump: u8,                // PDA bump
}

#[account]
pub struct StakeConfig {
    pub admin: Pubkey,           // Admin authority
    pub min_stake_amount: u64,   // Minimum stake amount
    pub max_stake_amount: u64,   // Maximum stake amount
    pub early_withdrawal_fee: u8, // % fee for early withdrawal (0-100)
    pub lock_options: Vec<u64>,  // Available lock periods in seconds
    pub bump: u8,                // PDA bump
}