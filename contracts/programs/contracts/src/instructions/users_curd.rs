use crate::state::users::{AddUser, DeleteUser, UpdateUser, User};
use anchor_lang::prelude::*;

pub fn add_user_details(
    ctx: Context<AddUser>,
    id: u64,
    name: String,
    hobbies: Vec<String>,
) -> Result<()> {
    msg!("Greetings from: {:?} add user", ctx.program_id);
    let user_pub_key = ctx.accounts.user.key();
    msg!(
        "User {user_pub_key} id {id} name {name} hobbies {:?}",
        hobbies
    );
    ctx.accounts
        .user_account
        .set_inner(User { id, name, hobbies });
    Ok(())
}

pub fn update_user_details(
    ctx: Context<UpdateUser>,
    name: String,
    hobbies: Vec<String>,
) -> Result<()> {
    msg!("Greetings from: {:?} update user", ctx.program_id);
    let user_pub_key = ctx.accounts.user.key();
    msg!("User {user_pub_key} name {name} hobbies {:?}", hobbies);
    let user = &mut ctx.accounts.user_account;
    user.name = name;
    user.hobbies = hobbies;
    Ok(())
}

pub fn delete_user_details(ctx: Context<DeleteUser>) -> Result<()> {
    msg!("Greetings from: {:?} delete user", ctx.program_id);
    Ok(())
}
