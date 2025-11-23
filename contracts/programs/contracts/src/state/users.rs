use crate::constants::{ANCHOR_DISCRIMINATOR_SIZE, USER_SEED};
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub id: u64,
    #[max_len(50)]
    pub name: String,
    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct AddUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + User::INIT_SPACE,
        seeds = [USER_SEED, user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, User>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct UpdateUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [USER_SEED, user.key().as_ref()],
        bump,
        realloc = ANCHOR_DISCRIMINATOR_SIZE + User::INIT_SPACE,
        realloc::payer = user, 
        realloc::zero = true,
    )]
    pub user_account: Account<'info, User>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeleteUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [USER_SEED, user.key().as_ref()],
        bump,
        close = user,
    )]
    pub user_account: Account<'info, User>,
    pub system_program: Program<'info, System>,
}

