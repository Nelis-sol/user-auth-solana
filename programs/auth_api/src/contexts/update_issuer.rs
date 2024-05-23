use crate::*;

#[derive(Accounts)]
pub struct UpdateIssuer<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"issuer".as_ref(), &authority.key().as_ref()], bump, constraint = authority.key() == issuer.issuer_key)]
    pub issuer: Account<'info, Issuer>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> UpdateIssuer<'_> {
    pub fn process(&mut self, status: u8) -> Result<()> {
        let Self {authority, issuer,..} = self;

        issuer.verified = status;

        Ok(())
    }
}