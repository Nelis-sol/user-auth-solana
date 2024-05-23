use crate::*;

#[derive(Accounts)]
#[instruction(em: Pubkey)]
pub struct UpdateIdentity<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"issuer".as_ref(), &authority.key().as_ref()], constraint = authority.key() == issuer.issuer_key, bump = issuer.bump)]
    pub issuer: Account<'info, Issuer>,
    #[account(mut, seeds = [b"identity".as_ref(), &authority.key().as_ref(), em.as_ref()], constraint = identity.issuer == issuer.issuer_id, bump = identity.bump)]
    pub identity: Account<'info, IdentityInfo>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> UpdateIdentity<'_> {
    pub fn process(&mut self, email: Pubkey, publickey: Pubkey) -> Result<()> {
        let Self {authority, issuer, identity,..} = self;

        // update timestamp
        let clock: Clock = Clock::get().unwrap();
        identity.update_ts = clock.unix_timestamp as u32;

        // update authority of pda
        identity.authority = publickey;

        Ok(())

    }

}
