pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8ZKiCKTeYGq6ArCHTsG4CzRTTJa67rxeYgNpstwavRbt");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        crate::instructions::increment::handle_increment(ctx)
    }

    pub fn create_profile(
    ctx: Context<CreateProfile>,
    name: String,
    bio: String,
) -> Result<()> {
    crate::instructions::profile::handle_create_profile(ctx, name, bio)
}

}
