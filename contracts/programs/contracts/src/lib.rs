pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("LEoKiTa1tp5ab3AZ4jGST7r99pX4Em8XGn3mcFpkqoE");

#[program]
pub mod contracts {
    use super::*;
    
    pub fn add_user(
        ctx: Context<AddUser>,
        id: u64,
        name: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        instructions::users_curd::add_user_details(ctx, id, name, hobbies)
    }

    pub fn update_user(ctx: Context<UpdateUser>, name: String, hobbies: Vec<String>) -> Result<()> {
        instructions::users_curd::update_user_details(ctx, name, hobbies)
    }

    pub fn delete_user(ctx: Context<DeleteUser>) -> Result<()> {
        instructions::users_curd::delete_user_details(ctx)
    }
}
