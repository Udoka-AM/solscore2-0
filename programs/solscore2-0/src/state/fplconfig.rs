use anchor_lang::prelude::*;

#[account]
pub struct FplUser {
    pub authority: Pubkey,       // User's wallet address
    pub fpl_id: String,          // User's FPL ID
    pub team_data: Vec<u8>,      // Serialized team data
    pub weekly_score: u32,       // Current weekly score
    pub total_score: u32,        // Total season score
    pub last_updated: i64,       // Timestamp of last update
    pub bump: u8,                // PDA bump
}

#[account]
pub struct FplGlobalState {
    pub admin: Pubkey,           // Admin authority
    pub current_gameweek: u8,    // Current FPL gameweek
    pub season_start: i64,       // Season start timestamp
    pub season_end: i64,         // Season end timestamp
    pub api_url: String,         // External FPL API URL
    pub bump: u8,                // PDA bump
}