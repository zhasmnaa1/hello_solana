use anchor_lang::prelude::*;

use crate::{constants::PROFILE_SEED, error::ErrorCode, state::Profile};

#[derive(Accounts)]
#[instruction(name: String, bio: String)]
pub struct CreateProfile<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Profile::INIT_SPACE,
        seeds = [PROFILE_SEED, authority.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,

    pub system_program: Program<'info, System>,
}

pub fn handle_create_profile(
    ctx: Context<CreateProfile>,
    name: String,
    bio: String,
) -> Result<()> {
    require!(name.len() <= 32, ErrorCode::NameTooLong);
    require!(bio.len() <= 200, ErrorCode::BioTooLong);
    
    let profile = &mut ctx.accounts.profile;
    profile.authority = ctx.accounts.authority.key();
    profile.name = name;
    profile.bio = bio;

    msg!("User profile created!");
    msg!("Authority: {}", profile.authority);
    msg!("Name: {}", profile.name);

    Ok(())
}

