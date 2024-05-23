use crate::*;

#[derive(Accounts)]
#[instruction(hash: Pubkey, shdw: u8, tos: u8)]
pub struct CreateApp<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"controller"], bump = controller.bump)]
    pub controller: Account<'info, Controller>,
    /// TODO: make sure client does not send a false identity PDA
    // possibly use issuer_id's -> find a way to use u32 in pda seeds
    #[account(has_one = authority)]
    pub identity: Account<'info, IdentityInfo>,
    #[account(init, payer = authority, space = 8 + mem::size_of::<App>(), seeds = [b"app".as_ref(), hash.as_ref()], bump)]
    pub app: Account<'info, App>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> CreateApp<'_> {
    pub fn process(&mut self, shdw: u8, tos: u8, bump: u8, action: bool) -> Result<()> {
        let Self {authority, controller, identity, app,..} = self;


        // set creation timestamp to now
        let clock: Clock = Clock::get().unwrap();
        app.ts = clock.unix_timestamp as u32;

        // add user id to the App PDA
        app.uid = identity.uid;

        // take the app id from the Controller and count +1 to create new app id for this entry
        app.aid = &controller.apps + 1;
        controller.apps += 1;

        // indicate the shadow storage account number where the app profile is stored
        app.shdw = shdw;

        // indicator for status of the app, for moderation purposes
        app.st = 1;

        // confirm / store that the user accepted the terms of service for the app
        // u8 instead of bool, because tos can change/up over time, so new permissions might be needed
        app.accepted_tos = tos;

        // identity is a PDA, so we store the bump
        app.bump = bump;



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
