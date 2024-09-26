use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_spl::token::{Token, TokenAccount};
use my_contract::initialize_account;
use my_contract::update_value;

#[tokio::test]
async fn test_initialize_account() {
    let program_id = Pubkey::new_unique();
    let (mut ctx, _user) = setup_context(program_id).await;

    let initial_value = 100;
    let tx = initialize_account::initialize(ctx.accounts.initialize_ctx(), initial_value).await;
    assert!(tx.is_ok());
    
    let account_data = ctx.accounts.my_account.load().await.unwrap();
    assert_eq!(account_data.value, initial_value);
}

#[tokio::test]
async fn test_update_value() {
    let program_id = Pubkey::new_unique();
    let (mut ctx, _user) = setup_context(program_id).await;

    // Initialize the account first
    let initial_value = 100;
    let _ = initialize_account::initialize(ctx.accounts.initialize_ctx(), initial_value).await.unwrap();

    // Update the value
    let new_value = 200;
    let tx = update_value::update_value(ctx.accounts.update_value_ctx(), new_value).await;
    assert!(tx.is_ok());
    
    let account_data = ctx.accounts.my_account.load().await.unwrap();
    assert_eq!(account_data.value, new_value);
}

async fn setup_context(program_id: Pubkey) -> (Context, User) {
    // logic
}
