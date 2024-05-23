use crate::*;


#[derive(Accounts)]
pub struct CreateChamber<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// TODO: make sure client does not send a false identity PDA
    // possibly use issuer_id's -> find a way to use u32 in pda seeds
    #[account(mut, has_one = authority)]
    pub identity: Account<'info, IdentityInfo>,
    #[account(init, payer = authority, space = 350, seeds = [b"chamber", identity.key().as_ref()], bump)]
    pub chamber: Account<'info, Chamber>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> CreateChamber<'_> {
    pub fn process(&mut self, bump: u8) -> Result<()> {
        let Self {authority, identity, chamber,..} = self;

        // set creation timestamp to now
        let clock: Clock = Clock::get().unwrap();
        chamber.ts = clock.unix_timestamp as u32;

        // add user id to the Chamber PDA
        chamber.uid = identity.uid;

        // add signer public key to sec field
        let vec_default = Vec::new();
        chamber.sec = vec_default;

        // identity is a PDA, so we store the bump
        self.chamber.bump = bump;

        Ok(())
    }
}



