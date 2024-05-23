use crate::*;

#[derive(Accounts)]
#[instruction(hash: Pubkey)]
pub struct UpdateGroup<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"controller"], bump = controller.bump)]
    pub controller: Account<'info, Controller>,
    /// TODO: make sure client does not send a false identity PDA
    // possibly use issuer_id's -> find a way to use u32 in pda seeds
    #[account(has_one = authority)]
    pub identity: Account<'info, IdentityInfo>,
    #[account(mut, seeds = [b"group".as_ref(), hash.as_ref()], constraint = identity.uid == group.uid, bump = group.bump)]
    pub group: Account<'info, Group>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> UpdateGroup<'_> {
    pub fn process(&mut self, action: bool, tos: Option<u8>) -> Result<()> {
        let Self {authority, controller, identity, group,..} = self;
        

        match tos {
            None => (),
            Some(accepted_tos) => {group.accepted_tos = accepted_tos;},
        }


        match action {
            false => (),
            true => {
                // how much lamports it costs to create a new profile
                let action_cost: u64 = 1;

                // get current time which we will use to see how much time has elapsed since last user update
                let clock: Clock = Clock::get().unwrap();
                let time_now: u32 = clock.unix_timestamp as u32;

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
