use anchor_lang::prelude::*;

declare_id!("DHZDcJbhgt57A114LYLycmyYn5s8Zr5jCnyVy2odP8aa");

#[program]
pub mod solscore2_0 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
