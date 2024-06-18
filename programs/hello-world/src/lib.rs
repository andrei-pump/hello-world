use anchor_lang::prelude::*;

declare_id!("6WWEZXoUdgrJ9AkbLdLBfZDKWVAXvEna8fRY8yrvvCNE");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
