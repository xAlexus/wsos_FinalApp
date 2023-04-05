

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};
use std::mem::size_of;
use anchor_lang::solana_program::entrypoint::ProgramResult;
declare_id!("3gQfmeiQb9zfP4Utg22Vn2RJvzAwx264oaNrctRNd91W");

const TEXT_LENGHT: usize = 1024;
const USER_NAME_LENGHT: usize = 100;
const USER_URL_LENGHT: usize = 255;

#[program]
pub mod programs {
    use super::*;

    pub fn create_state(ctx: Context<CreateState>) -> ProgramResult{
        let state = &mut ctx.accounts.state;
        state.authority = ctx.accounts.authority.key();
        state.post_count = 0;
        Ok(())
    }
    pub fn create_post(ctx: Context<CreatePost>,
    text: String,
    poster_name: String,
    poster_url: String,) -> ProgramResult{
        let state = &mut ctx.accounts.state;

        let post = &mut ctx.accounts.post;

        post.text = text;
        post.poster_name = poster_name;
        post.poster_url = poster_url;
      //post.comment_count = 0;
        post.index = state.post_count;
        post.post_time = ctx.accounts.clock.unix_timestamp;

        state.post_count += 1;
        Ok(())

    }
}

#[derive(Accounts)]
pub struct CreateState<'info> {
    #[account(
        init, 
        seeds = [b"state".as_ref()],
        bump,
        payer = authority,
        space = size_of::<StateAccount>()
    )]
    pub state: Account<'info, StateAccount>,

    //Authority (Signer of the Transaction)
    #[account(mut)]
    pub authority: Signer<'info>,

    //System Program
    pub system_program: UncheckedAccount<'info>,
    
    //Token Program
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>
}
#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(mut, seeds = [b"state".as_ref()],bump)]
    pub state: Account<'info, StateAccount>,

    //Authenticate Post Account
    #[account(
        init,
        seeds = [b"post".as_ref(),state.post_count.to_be_bytes().as_ref()],
        bump,
        payer = authority,
        space = size_of::<PostAccount>() + USER_NAME_LENGHT + USER_URL_LENGHT + TEXT_LENGHT
    )]
    pub post: Account<'info, PostAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: UncheckedAccount<'info>,
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct StateAccount {
    pub authority: Pubkey,
    pub post_count: u64,
}
//Post Account Struct
#[account]
pub struct PostAccount {
    pub authority: Pubkey,
    pub text: String,
    pub poster_name: String,
    pub poster_url: String,
    pub index: u64,
    pub post_time: i64,
}
