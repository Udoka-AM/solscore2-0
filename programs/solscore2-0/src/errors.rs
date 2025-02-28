use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid FPL ID")]
    InvalidFplId,
    
    #[msg("Invalid stake amount")]
    InvalidStakeAmount,
    
    #[msg("Invalid lock period")]
    InvalidLockPeriod,
    
    #[msg("Unauthorized access")]
    UnauthorizedAccess,
    
    #[msg("Stake not active")]
    StakeNotActive,
    
    #[msg("Insufficient funds")]
    InsufficientFunds,
    
    #[msg("No rewards available")]
    NoRewardsAvailable,
    
    #[msg("Invalid withdrawal amount")]
    InvalidWithdrawalAmount,
    
    #[msg("Too early to claim rewards")]
    TooEarlyToClaim,

    #[msg("Invalid reward parameter")]
    InvalidRewardParameter,
    
    #[msg("Invalid treasury parameter")]
    InvalidTreasuryParameter,
    
    #[msg("Invalid deposit amount")]
    InvalidDepositAmount,
    
    #[msg("Exceeds withdrawal limit")]
    ExceedsWithdrawalLimit,
}