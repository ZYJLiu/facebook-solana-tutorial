use anchor_lang::prelude::*;
use anchor_spl::token::Token;

declare_id!("ARmGAxXNSbpKb4DpHrZpjPinPf47sKZu2YnuEG9S4gf4");

const TEXT_LENGTH: usize = 1024;
const USER_NAME_LENGTH: usize = 100; 
const USER_URL_LENGTH: usize = 255; 


#[program]
pub mod programs {
    use super::*;

    pub fn create_state(
        ctx: Context<CreateState>
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;

        state.authority = ctx.accounts.authority.key();
        state.post_count = 0;
        Ok(())
    }

    pub fn create_post(
        ctx: Context<CreatePost>,
        text: String,
        post_name: String,
        poster_url: String,
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let post = &mut ctx.accounts.post;

        post.authority = ctx.accounts.authority.key();
        post.text = text;
        post.poster_name = post_name;
        post.poster_url = poster_url;
        post.comment_count = 0;
        post.index = state.post_count;
        post.post_timer = ctx.accounts.clock.unix_timestamp;
        state.post_count += 1;

        Ok(())
    }

    /// Create comment for post
    /// @param text:            text of comment
    /// @param commenter_name:  name of comment creator
    /// @param commenter_url:   url of comment creator avatar
    pub fn create_comment(
        ctx: Context<CreateComment>,
        text: String,
        commenter_name: String,
        commenter_url: String,
    ) -> Result<()> {

        // Get post
        let post = &mut ctx.accounts.post;

        // Get comment
        let comment = &mut ctx.accounts.comment;
        // Set authority to comment
        comment.authority = ctx.accounts.authority.key();
        // Set comment text
        comment.text = text;
        // Set commenter name
        comment.commenter_name = commenter_name;
        // Set commenter url
        comment.commenter_url = commenter_url;
        // Set comment index to post's comment count
        comment.index = post.comment_count;
        // Set post time
        comment.post_time = ctx.accounts.clock.unix_timestamp;

        // Increase post's comment count by 1
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
        space = 8 + 1000 //PLACEHOLDER SPACE
    )]
    pub state: Account<'info, StateAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

    // #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,

    pub clock: Sysvar <'info, Clock>,
}

#[derive(Accounts)]
pub struct CreatePost<'info>{
    #[account(
        mut, 
        seeds = [b"state".as_ref()],
        bump
    )]
    pub state: Account<'info, StateAccount>,

    #[account(
        init,
        seeds = [b"post".as_ref(), state.post_count.to_be_bytes().as_ref()],
        bump,
        payer = authority,
        space = 8 + USER_URL_LENGTH + USER_NAME_LENGTH + TEXT_LENGTH + 1000 // PLACEHOLDER SPACE
    )]
    pub post: Account<'info, PostAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

    // #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,

    pub clock: Sysvar <'info, Clock>,
}

/// CreateComment context
#[derive(Accounts)]
pub struct CreateComment<'info> {
    // Authenticate post account
    #[account(mut, seeds = [b"post".as_ref(), post.index.to_be_bytes().as_ref()], bump)]
    pub post: Account<'info, PostAccount>,

    // Authenticate comment account
    #[account(
        init,
        // Post account use string "comment", index of post and index of comment per post as seeds
        seeds = [b"comment".as_ref(), post.index.to_be_bytes().as_ref(), post.comment_count.to_be_bytes().as_ref()],
        bump,
        payer = authority,
        space = 8 + TEXT_LENGTH + USER_NAME_LENGTH + USER_URL_LENGTH + 1000
    )]
    pub comment: Account<'info, CommentAccount>,

    // Authority (this is signer who paid transaction fee)
    #[account(mut)]
    pub authority: Signer<'info>,

    /// System program
    pub system_program: Program<'info, System>,

    // Token program
    pub token_program: Program<'info, Token>,

    // Clock to save time
    pub clock: Sysvar<'info, Clock>,
}


#[account]
pub struct StateAccount{
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
    pub post_timer: i64,
}

// Comment Account Structure
#[account]
pub struct CommentAccount {
    // Signer address
    pub authority: Pubkey,

    // Comment text
    pub text: String,

    // commenter_name
    pub commenter_name: String,

    // commenter_url
    pub commenter_url: String,

    // Comment index
    pub index: u64,

    // Post time
    pub post_time: i64,
}
