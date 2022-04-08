use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};
use std::mem::size_of;

declare_id!("EsgFRxZjRnsgDuY7retJHAqmJzpvZVkmojMaS9gQo3Ao");

const TEXT_LENGTH: usize = 1024;
const USER_NAME_LENGTH: usize = 100;
const USER_URL_LENGTH: usize = 255;

#[program]
pub mod facebook_sol {
    use super::*;

    pub fn create_state(
        ctx: Context<CreateState>,
    ) -> ProgramResult {
        let state = &mut ctx.accounts.state;
        state.authority = ctx.accounts.authority.key();
        state.post_count = 0;
        Ok(())
    }

    pub fn create_post(
        ctx: Context<CreatePost>,
        text: String,
        poster_name: String,
        poster_url: String,
    ) -> ProgramResult {
        let state = &mut ctx.accounts.state;

        let post = &mut ctx.accounts.post;
        post.authority = ctx.accounts.authority.key();
        post.text = text;
        post.poster_name = poster_name;
        post.poster_url = poster_url;
        post.comment_count = 0;
        post.index = state.post_count;
        post.post_time = ctx.accounts.clock.unix_timestamp;

        state.post_count += 1;
        Ok(())
    }

    pub fn create_comment(
        ctx: Context<CreateComment>,
        text: String,
        commenter_name: String,
        commenter_url: String,
    ) -> ProgramResult {

        let post = &mut ctx.accounts.post;

        let comment = &mut ctx.accounts.comment;

        comment.authority = ctx.accounts.authority.key();

        comment.text = text;

        comment.commenter_name = commenter_name;

        comment.commenter_url = commenter_url;

        comment.index = post.comment_count;

        comment.post_time = ctx.accounts.clock.unix_timestamp;

        post.comment_count += 1;
        
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
        space = size_of::<StateAccount>() + 8
    )]
    pub state: Account<'info, StateAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: UncheckedAccount<'info>,

    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(mut, seeds = [b"state".as_ref()], bump)]
    pub state: Account<'info, StateAccount>,

    #[account(
        init,
        seeds = [b"post".as_ref(), state.post_count.to_be_bytes().as_ref()],
        bump,
        payer = authority,
        space = size_of::<PostAccount>() + TEXT_LENGTH + USER_NAME_LENGTH + USER_URL_LENGTH
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

#[account]
pub struct PostAccount {
    pub authority: Pubkey,
    pub text: String,
    pub poster_name: String,
    pub poster_url: String,
    pub comment_count: u64,
    pub index: u64,
    pub post_time: i64,
}

#[derive(Accounts)]
pub struct CreateComment<'info> {
    #[account(mut, seeds = [b"post".as_ref(), post.index.to_be_bytes().as_ref()], bump)]
    pub post: Account<'info, PostAccount>,

    #[account(
        init,
        seeds = [b"comment".as_ref(), post.index.to_be_bytes().as_ref(), post.comment_count.to_be_bytes().as_ref()], bump,
        payer = authority,
        space = size_of::<CommentAccount>() + TEXT_LENGTH + USER_NAME_LENGTH + USER_URL_LENGTH
    )]
    pub comment: Account<'info, CommentAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: UncheckedAccount<'info>,

    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,

    pub clock: Sysvar<'info,Clock>,
}



#[account]
pub struct CommentAccount {
    pub authority: Pubkey,

    pub text: String,

    pub commenter_name: String,

    pub commenter_url: String,

    pub index: u64,

    pub post_time: i64,
}