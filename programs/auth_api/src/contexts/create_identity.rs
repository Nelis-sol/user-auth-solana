use crate::*;

#[derive(Accounts)]
#[instruction(em: Pubkey, tos: u8)]
pub struct CreateIdentity<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"controller"], bump = controller.bump)]
    pub controller: Account<'info, Controller>,
    #[account(mut, seeds = [b"issuer".as_ref(), &authority.key().as_ref()], bump = issuer.bump)]
    pub issuer: Account<'info, Issuer>,
    #[account(init, payer = authority, space = 8 + mem::size_of::<IdentityInfo>(), seeds = [b"identity".as_ref(), &authority.key().as_ref(), em.as_ref()], bump)]
    pub identity: Account<'info, IdentityInfo>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> CreateIdentity<'_> {
    pub fn process(&mut self, email: Pubkey, publickey: Pubkey, tos: u8, bump: u8) -> Result<()> {
        let Self {authority, controller, issuer, identity,..} = self;

        // set creation and update timestamp to now
        let clock: Clock = Clock::get().unwrap();
        identity.creation_ts = clock.unix_timestamp as u32;
        identity.update_ts = clock.unix_timestamp as u32;

        // issuer of the identity
        identity.issuer = issuer.issuer_id;

        // mark as verified if issuer is verified
        if issuer.verified == 1 {
            identity.verified = 1;
        }

        // authority of the session, changes on new login
        identity.authority = publickey;

        // take the user id from the Controller and count +1 to create new user id for this entry
        identity.uid = &controller.users + 1;
        controller.users += 1;

        // counter to keep track of user actions per time unit
        identity.actions = 0;

        // confirm / store that the user accepted the terms of service
        // u8 instead of bool, because tos can change/up over time, so new permissions might be needed
        identity.accepted_tos = tos;

        // identity is a PDA, so we store the bump
        identity.bump = bump;

        Ok(())

    }

}
