use crate::*;

#[derive(Accounts)]
pub struct CreateIssuer<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"controller"], bump = controller.bump)]
    pub controller: Account<'info, Controller>,
    #[account(init, payer = authority, space = 8 + mem::size_of::<Issuer>(), seeds = [b"issuer".as_ref(), &authority.key().as_ref()], bump)]
    pub issuer: Account<'info, Issuer>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> CreateIssuer<'_> {
    pub fn process(&mut self, bump: u8) -> Result<()> {
        let Self {authority, issuer, controller,..} = self;

        // store issuer key as authority of the pda
        issuer.issuer_key = *authority.key;

        // take the issuer id from the Controller and count +1 to create new issuer id for this entry
        issuer.issuer_id = &controller.issuer + 1;
        controller.issuer += 1;

        // mark issuer as verified
        issuer.verified = 0;

        // identity is a PDA, so we store the bump
        issuer.bump = bump;

        Ok(())

    }

}
