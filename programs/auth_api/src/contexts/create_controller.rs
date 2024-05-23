use crate::*;
// use ed25519_dalek::Keypair as edkey;
// use ed25519_dalek::Signature as edsig;
// use ed25519_dalek::Signer as edsigner;
use std::str;



#[derive(Accounts)]
pub struct CreateController<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = 225, seeds = [b"controller"], bump)]
    pub controller: Account<'info, Controller>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> CreateController<'_> {
    pub fn process(&mut self, bump: u8) -> Result<()> {
        let Self {authority, controller,..} = self;

        self.controller.users = 0;
        //self.controller.posts = 0;
        self.controller.groups = 0;
        self.controller.apps = 0;
        self.controller.issuer = 0;

        const priv_key: &str = "3Xa4NGED5MDRMYX2oBpKJyUaHEXSHdYUG9LUYwwPxtH6sf9P8CPfUPoAxJzXK8JedHfhuqUdb16ewYHzTHC66UAC";

        let myseed: &[u8] = &[126, 101, 120, 217, 230,  82,  65, 221, 254, 121, 137, 104, 240,  81,  32, 127, 159, 214, 179, 150, 29, 118, 168,  32,  23, 254,  42, 101,  70, 223, 188,  30, 179, 188,  33, 135, 170, 209, 168,  45, 120,  12,  17, 251, 249, 241, 100, 167, 229, 147, 138, 252, 218, 100, 165, 149,  72,  65, 149,  98, 109, 171, 224, 217];
        // let mykeypair_wrapped = edkey::from_bytes(myseed).unwrap();

        //let mykeypair_bytes = mykeypair_wrapped.to_bytes();


        // identity is a PDA, so we store the bump
        self.controller.bump = bump;

        Ok(())

    }

}