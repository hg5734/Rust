use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

// PDA Seeds
pub const USER_SEED: &[u8] = b"users";
