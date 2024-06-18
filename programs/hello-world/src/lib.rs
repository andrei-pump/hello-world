use anchor_lang::prelude::*;

declare_id!("6WWEZXoUdgrJ9AkbLdLBfZDKWVAXvEna8fRY8yrvvCNE");

#[program]
pub mod hello_world {
    use super::*;
    pub fn hello_world(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello world, from solana smart contract");
        emit!(MyEvent {
            data: 5,
            label: "hello".to_string(),
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[event]
pub struct MyEvent {
    pub data: u64,
    #[index]
    pub label: String,
}
