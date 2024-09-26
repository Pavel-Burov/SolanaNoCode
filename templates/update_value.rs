use anchor_lang::prelude::*;

#[program]
pub mod update_value {
    use super::*;

    pub fn update_value(ctx: Context<UpdateValue>, new_value: u64) -> Result<()> {
        let account = &mut ctx.accounts.my_account;
        account.value = new_value;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateValue<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[account]
pub struct MyAccount {
    pub value: u64,
}
