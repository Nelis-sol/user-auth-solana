use crate::*;

#[derive(Accounts)]
#[instruction(hash: Pubkey, shdw: u8, tos: u8, aid: Option<u16>)]
pub struct CreateProfile<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"controller"], bump = controller.bump)]
    pub controller: Account<'info, Controller>,
    /// TODO: make sure client does not send a false identity PDA
    // possibly use issuer_id's -> find a way to use u32 in pda seeds
    #[account(mut, has_one = authority)]
    pub identity: Account<'info, IdentityInfo>,
    #[account(init, payer = authority, space = 8 + mem::size_of::<Profile>(), seeds = [b"profile".as_ref(), hash.as_ref()], bump)]
    pub profile: Account<'info, Profile>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> CreateProfile<'_> {
    pub fn process(&mut self, shdw: u8, tos: u8, bump: u8, action: bool, aid: Option<u32>) -> Result<()> {
        let Self {authority, controller, identity, profile,..} = self;

        // set creation timestamp to now
        let clock: Clock = Clock::get().unwrap();
        profile.ts = clock.unix_timestamp as u32;

        // assign user profile to a specific app
        // default = 0
        let app_id: u32;
        match aid {
            None => {profile.aid = 0;},
            Some(appid) => {profile.aid = appid;},
        }

        // add user id to the App PDA
        profile.uid = identity.uid;

        // indicate the shadow storage account number where the app profile is stored
        profile.shdw = shdw;

        // indicator for status of the app, for moderation purposes
        profile.st = 1;

        // role indicator (e.g. user, company, developer)
        // default = 0
        profile.role = 0;

        // confirm / store that the user accepted the terms of service for the app
        // u8 instead of bool, because tos can change/up over time, so new permissions might be needed
        profile.accepted_tos = tos;

        // identity is a PDA, so we store the bump
        profile.bump = bump;


        match action {
            false => (),
            true => {
                // how much lamports it costs to create a new profile
                let action_cost: u64 = 1;

                // get current time which we will use to see how much time has elapsed since last user update
                let clock: Clock = Clock::get().unwrap();
                let time_now: u32 = clock.unix_timestamp as u32;

                if identity.verified == 1 {
                    // check if user has done less than 255 actions
                    // if yes, we fund his transaction
                    if identity.actions < 255 {

                        // transfer SOL tokens
                        **controller.to_account_info().try_borrow_mut_lamports()? -= action_cost;
                        **authority.try_borrow_mut_lamports()? += action_cost;

                        // check if user has performed 255 and last update was not less than 24 hours ago
                        // we fund up to 255 user actions per 24 hours
                    } else if identity.actions == 255 && (time_now - identity.update_ts) < 86400 {
                    
                        // reset action to 1
                        identity.actions = 1;

                        // transfer SOL tokens
                        **controller.to_account_info().try_borrow_mut_lamports()? -= action_cost;
                        **authority.try_borrow_mut_lamports()? += action_cost;

                    } else {
                        // user is most likely spamming, throw error
                        return Err(ErrorCode::SlowDown.into())
                    }
                } else {
                    // transfer SOL tokens
                    **identity.to_account_info().try_borrow_mut_lamports()? -= action_cost;
                    **authority.try_borrow_mut_lamports()? += action_cost;
                }
            }
        }

        Ok(())
    }
}


#[error_code]
pub enum ErrorCode {
    #[msg("Funding for user action exceeded")]
    SlowDown,
}
