use anchor_lang::prelude::*;

pub mod state;
pub use state::*;

declare_id!("J5v18cnHYUGTF1wLHQvkTkJhLebNUxhRiijt6pBjsqTy");

#[program]
pub mod nf_minter {
    use super::*;

    pub fn create_token(ctx: Context<Initialize>) -> Result<()> {


        Ok(())
    }
}


#[derive(Accounts)]
pub struct Initialize {}
