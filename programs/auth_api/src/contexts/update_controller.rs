use crate::*;

#[derive(Accounts)]
pub struct UpdateController<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"controller"], bump = controller.bump)]
    pub controller: Account<'info, Controller>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}