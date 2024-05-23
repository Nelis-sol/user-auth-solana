use crate::*;

#[derive(Accounts)]
#[instruction(hash: Pubkey, is: Pubkey, em: Pubkey)]
pub struct UpdatePost<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(seeds = [b"identity".as_ref(), is.as_ref(), em.as_ref()], bump = identity.bump, has_one = authority)]
    pub identity: Account<'info, IdentityInfo>,
    #[account(mut, seeds = [b"post".as_ref(), hash.as_ref()], bump = post.bump, constraint = identity.uid == post.uid)]
    pub post: Account<'info, Post>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}