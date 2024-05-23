use anchor_lang::prelude::*;
use std::mem;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::system_program;

declare_id!("8sfYb3xx2gNtwtuwg6UzcpPx4FZK1WJJbYZbPgy1hrQJ");

pub mod contexts;
pub mod states;

pub use contexts::*;
pub use states::*;

#[program]
pub mod auth_api {
    use super::*;

    pub fn create_controller(ctx: Context<CreateController>) -> Result<()> {
        let bump = *ctx.bumps.get("controller").unwrap();
        ctx.accounts.process(bump)
    }

    pub fn create_issuer(ctx: Context<CreateIssuer>) -> Result<()> {
        let bump = *ctx.bumps.get("issuer").unwrap();
        ctx.accounts.process(bump)
    }

    pub fn create_identity(ctx: Context<CreateIdentity>, email: Pubkey, publickey: Pubkey, tos: u8) -> Result<()> {
        let bump = *ctx.bumps.get("identity").unwrap();
        ctx.accounts.process(email, publickey, tos, bump)
    }

    pub fn update_identity(ctx: Context<UpdateIdentity>, email: Pubkey, publickey: Pubkey) -> Result<()> {
        ctx.accounts.process(email, publickey)
    }

    pub fn create_app(ctx: Context<CreateApp>, _hash: Pubkey, shdw: u8, tos: u8, action: bool) -> Result<()> {
        let bump = *ctx.bumps.get("app").unwrap();
        ctx.accounts.process(shdw, tos, bump, action)
    }

    pub fn create_profile(ctx: Context<CreateProfile>, _hash: Pubkey, shdw: u8, tos: u8, action: bool, aid: Option<u32>) -> Result<()> {
        let bump = *ctx.bumps.get("profile").unwrap();
        ctx.accounts.process(shdw, tos, bump, action, aid)
    }

    pub fn create_group(ctx: Context<CreateGroup>, _hash: Pubkey, shdw: u8, parent_type: u8, parent: u32, tos: u8, action: bool) -> Result<()> {
        let bump = *ctx.bumps.get("group").unwrap();
        ctx.accounts.process(shdw, parent_type, parent, tos, bump, action)
    }

    pub fn create_post(ctx: Context<CreatePost>, _hash: Pubkey, parent_type: u8, parent: u32, action: bool) -> Result<()> {
        let bump = *ctx.bumps.get("post").unwrap();
        ctx.accounts.process(parent_type, parent, bump, action)
    }

    pub fn create_chamber(ctx: Context<CreateChamber>) -> Result<()> {
        let bump = *ctx.bumps.get("chamber").unwrap();
        ctx.accounts.process(bump)
    }

    pub fn update_chamber(ctx: Context<UpdateChamber>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn delete_post(ctx: Context<DeletePost>, _hash: Pubkey) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn delete_group(ctx: Context<DeleteGroup>, _hash: Pubkey) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn delete_profile(ctx: Context<DeleteProfile>, _hash: Pubkey) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn delete_app(ctx: Context<DeleteApp>, _hash: Pubkey) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn update_group(ctx: Context<UpdateGroup>, _hash: Pubkey, action: bool, tos: Option<u8>) -> Result<()> {
        ctx.accounts.process(action, tos)
    }

    pub fn update_profile(ctx: Context<UpdateProfile>, _hash: Pubkey, action: bool, tos: Option<u8>) -> Result<()> {
        ctx.accounts.process(action, tos)
    }

    pub fn update_app(ctx: Context<UpdateApp>, _hash: Pubkey, action: bool, tos: Option<u8>) -> Result<()> {
        ctx.accounts.process(action, tos)
    }

    pub fn update_issuer(ctx: Context<UpdateIssuer>, status: u8) -> Result<()> {
        ctx.accounts.process(status)
    }

}







