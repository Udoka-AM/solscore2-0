use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod errors;

use instructions::*;

declare_id!("DHZDcJbhgt57A114LYLycmyYn5s8Zr5jCnyVy2odP8aa");

#[program]

pub mod solscore {
    use super::*;

    // FPL Manager Instructions
    pub fn fplinit(ctx: Context<InitializeFplGlobal>, params: FplGlobalParams) -> Result<()> {
        instructions::fpl::initialize_fpl_global(ctx, params)
    }

    pub fn userreg(ctx: Context<RegisterFplUser>, fpl_id: String) -> Result<()> {
        instructions::fpl::register_fpl_user(ctx, fpl_id)
    }

     pub fn update_fpl_data(ctx: Context<UpdateFplData>) -> Result<()> {
        instructions::fpl::update_fpl_data(ctx)
     }

    
    // Stake Instructions

   pub fn initialize_stake_config(ctx: Context<InitializeStakeConfig>, params: StakeConfigParams) -> Result<()> {
    instructions::stake::initialize_stake_config(ctx, params)
     }

    pub fn stake(ctx: Context<CreateStake>, amount: u64, lock_period: u64) -> Result<()> {
        instructions::stake::create_stake(ctx, amount, lock_period)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        instructions::stake::unstake(ctx)
    }

    // Reward Instructions
    pub fn initrewardsconfig(ctx: Context<InitializeRewardConfig>, params: RewardConfigParams) -> Result<()> {
        instructions::reward::initrewardsconfig(ctx, params)
    }

    pub fn initrewardpool(ctx: Context<InitializeRewardPool>) -> Result<()> {
        instructions::reward::initrewardpool(ctx)
    }

    pub fn claimrewards(ctx: Context<ClaimRewards>) -> Result<()> {
        instructions::reward::claimrewards(ctx)
    }

    pub fn distrewards(ctx: Context<DistributeRewards>) -> Result<()> {
        instructions::reward::distrewards(ctx)
    }

    // Treasury Instructions
    pub fn initialize_reward_config(ctx: Context<InitializeRewardConfig>, params: RewardConfigParams) -> Result<()> {
        instructions::reward::initialize_reward_config(ctx, params)
    }
    
    pub fn initialize_reward_pool(ctx: Context<InitializeRewardPool>) -> Result<()> {
        instructions::reward::initialize_reward_pool(ctx)
    }
    
    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        instructions::reward::claim_rewards(ctx)
    }
    
    pub fn distribute_rewards(ctx: Context<DistributeRewards>, amount: u64) -> Result<()> {
        instructions::reward::distribute_rewards(ctx, amount)
    }
    
    // Treasury Program Instructions
    pub fn treasuryinit(ctx: Context<InitializeTreasury>, params: TreasuryParams) -> Result<()> {
        instructions::treasury::treasuryinit(ctx, params)
    }
    
    pub fn treasurydeposit(ctx: Context<DepositToTreasury>, amount: u64) -> Result<()> {
        instructions::treasury::treasurydeposit(ctx, amount)
    }
    
    pub fn withdraw(ctx: Context<WithdrawTreasury>, amount: u64) -> Result<()> {
        instructions::treasury::withdraw(ctx, amount)
    }
    
    pub fn updatetreasuryconfig(
        ctx: Context<UpdateTreasuryConfig>, 
        new_protocol_fee: Option<u8>, 
        new_reserve_percentage: Option<u8>
    ) -> Result<()> {
        instructions::treasury::updatetreasuryconfig(ctx, new_protocol_fee, new_reserve_percentage)
    }

    
        
        // ... update locktime for tests
        
        pub fn update_lock_timestamp_for_testing(
            ctx: Context<UpdateLockTimestamp>,
            new_timestamp: i64,
        ) -> Result<()> {
            // Only allow this function in non-production builds or add other safeguards
            #[cfg(not(feature = "production"))]
            {
                let user_stake_account = &mut ctx.accounts.user_stake_account;
                user_stake_account.lock_end_timestamp = new_timestamp;
                
                Ok(())
            }
            
            #[cfg(feature = "production")]
            {
                return Err(ProgramError::InvalidInstructionData.into());
            }
        }
        
        // ... update locktime struct
    
    #[derive(Accounts)]
    pub struct UpdateLockTimestamp<'info> {
        #[account(
            constraint = stakeConfig.admin == admin.key(),
        )]
        pub admin: Signer<'info>,
        
        #[account(
            mut,
        )]
        pub user_stake_account: Account<'info, UserStakeAccount>,
    }
}