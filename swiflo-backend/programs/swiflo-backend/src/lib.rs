use anchor_lang::prelude::*;

declare_id!("BANUjtphR7VU94S1q6fSpZTHhxj7foBqQQKt8uwCM9gY");

#[program]
pub mod swiflo_backend {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
