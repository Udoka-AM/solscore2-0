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

   // pub fn update_fpl_data(ctx: Context<UpdateFplData>) -> Result<()> {
   //     instructions::fpl::update_fpl_data(ctx)
   //  }

    // Stake Instructions

  //  pub fn initialize_stake_config(ctx: Context<InitializeStakeConfig>, params: StakeConfigParams) -> Result<()> {
    //    instructions::stake::initialize_stake_config(ctx, params)
   // }

    pub fn stake(ctx: Context<CreateStake>, amount: u64, lock_period: u64) -> Result<()> {
        instructions::stake::create_stake(ctx, amount, lock_period)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        instructions::stake::unstake(ctx)
    }

    // Reward Instructions
    pub fn initrewardsconfig(ctx: Context<InitializeRewardConfig>, params: RewardConfigParams) -> Result<()> {
        instructions::reward::initialize_reward_config(ctx, params)
    }

    pub fn initrewardpool(ctx: Context<InitializeRewardPool>) -> Result<()> {
        instructions::reward::initialize_reward_pool(ctx)
    }

    pub fn claimrewards(ctx: Context<ClaimRewards>) -> Result<()> {
        instructions::reward::claim_rewards(ctx)
    }

    pub fn distrewards(ctx: Context<DistributeRewards>) -> Result<()> {
        instructions::reward::distribute_rewards(ctx)
    }

    // Treasury Instructions
   /*  pub fn initialize_treasury(ctx: Context<InitializeTreasury>, params: TreasuryParams) -> Result<()> {
        instructions::treasury::initialize_treasury(ctx, params)
    }

    pub fn withdraw_treasury(ctx: Context<WithdrawTreasury>, amount: u64) -> Result<()> {
        instructions::treasury::withdraw_treasury(ctx, amount)
    }*/
    
}

#[derive(Accounts)]
pub struct Initialize {}
