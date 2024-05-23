use crate::*;

#[derive(Accounts)]
#[instruction(em: Pubkey)]
pub struct DeleteIdentity<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(seeds = [b"issuer".as_ref(), &authority.key().as_ref()], bump = issuer.bump, constraint = authority.key() == issuer.issuer_key)]
    pub issuer: Account<'info, Issuer>,
    #[account(mut, seeds = [b"identity".as_ref(), &authority.key().as_ref(), em.as_ref()], bump = identity.bump, close = authority)]
    pub identity: Account<'info, IdentityInfo>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}