use anchor_lang::prelude::*;

#[account]
pub struct RewardPool {
    pub total_rewards: u64,      // Total reward pool in SOL (lamports)
    pub distributed_rewards: u64, // Total distributed rewards
    pub admin: Pubkey,           // Admin authority
    pub bump: u8,                // PDA bump
}

#[account]
pub struct RewardConfig {
    pub admin: Pubkey,           // Admin authority
    pub base_apy: u8,            // Base APY percentage
    pub score_multiplier: u8,    // Multiplier for FPL scores
    pub distribution_frequency: u64, // Distribution frequency in seconds
    pub bump: u8,                // PDA bump
}