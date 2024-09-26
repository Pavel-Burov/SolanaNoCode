use anchor_lang::prelude::*;

declare_id!("{Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS}");

#[program]
pub mod my_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_value: u64) -> Result<()> {
        let account = &mut ctx.accounts.my_account;
        account.value = initial_value;
        Ok(())
    }

    pub fn update_value(ctx: Context<UpdateValue>, new_value: u64) -> Result<()> {
        let account = &mut ctx.accounts.my_account;
        account.value = new_value;
        Ok(())
    }
}

#[account]
pub struct MyAccount {
    pub value: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateValue<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}
