use crate::*;

#[derive(Accounts)]
#[instruction(hash: Pubkey)]
pub struct DeleteApp<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"controller"], bump = controller.bump)]
    pub controller: Account<'info, Controller>,
    /// TODO: make sure client does not send a false identity PDA
    // possibly use issuer_id's -> find a way to use u32 in pda seeds
    #[account(has_one = authority)]
    pub identity: Account<'info, IdentityInfo>,
    #[account(mut, seeds = [b"app".as_ref(), hash.as_ref()], constraint = identity.uid == app.uid, bump = app.bump, close = controller)]
    pub app: Account<'info, App>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> DeleteApp<'_> {
    pub fn process(&mut self) -> Result<()> {
        Ok(())
    }
}