use crate::*;

#[account]
pub struct Issuer {   
    pub issuer_key: Pubkey, // 32 byte - public key
    pub issuer_id: u32,     // 4 byte - unique id
    pub verified: u8,       // 1 byte - indicator if issuer is trustworthy company (e.g. Auth0)
    pub bump: u8,           // 1 byte - bump
}

#[account]
pub struct IdentityInfo {   
    pub creation_ts: u32,   // 4 byte - timestamp
    pub update_ts: u32,     // 4 byte - timestamp
    pub issuer: u32,        // 4 byte - public key
    pub verified: u8,       // 1 byte - is the issuer a verified identity
    pub authority: Pubkey,  // 32 byte - public key
    pub uid: u32,           // 4 byte - user id
    pub actions: u8,         // 1 byte - counter to limit user actions per time unit, to prevent spam
    pub accepted_tos: u8,   // 1 byte - did user accept terms of service
    pub bump: u8,           // 1 byte - bump
}

#[account]
pub struct Controller {   
    pub users: u32,     // doubles as count of users and user id's
    //pub posts: u32,     // doubles as count of posts and post id's
    pub groups: u32,    // doubles as count of groups and group id's
    pub apps: u32,      // doubles as count of apps and app id's
    pub issuer: u32,    // doubles as count of issuers and issuer id's
    pub bump: u8,  
}

#[account]
pub struct App {   
    pub ts: u32,        // 4 byte - timestamp
    pub uid: u32,       // 4 byte - user id
    pub aid: u32,       // 2 byte - app id
    pub shdw: u8,       // 1 byte - shadow drive counter, the Xth storage account
    pub st: u8,         // 1 byte - status of app (default = 1)
    pub accepted_tos: u8,   // 1 byte - accepted terms of service
    pub bump: u8,  
}

#[account]
pub struct Profile {
    pub ts: u32,        // 4 byte - timestamp
    pub aid: u32,       // 2 byte - app id
    pub uid: u32,       // 4 byte - user id
    pub shdw: u8,       // 1 byte - shadow drive counter, the Xth storage account
    pub st: u8,         // 1 byte - status
    pub role: u8,       // 1 byte - role indicator (e.g. user, company, developer)
    pub accepted_tos: u8,   // 1 byte - accepted terms of service
    pub bump: u8,
}

#[account]
pub struct Group {
    pub ts: u32,        // 4 byte - timestamp
    pub uid: u32,       // 4 byte - user id
    pub gid: u32,       // 4 byte - group id
    pub parent: u32,    // 4 byte - parent id (e.g. app or group id)
    pub parent_type: u8,    // 1 byte - defines the type of parent (e.g. 1 = app id or 1 = group id)
    pub shdw: u8,       // 1 byte - shadow drive counter, the Xth storage account
    pub st: u8,         // 1 byte - status
    pub accepted_tos: u8,   // 1 byte - accepted terms of service
    pub bump: u8,
}


#[account]
pub struct Post {
    pub ts: u32,        // 4 byte - timestamp
    pub uid: u32,       // 4 byte - user id
    //pub pid: u32,       // 4 byte - post id
    pub parent: u32,    // 4 byte - parent id (e.g. app or group id)
    pub parent_type: u8,    // 1 byte - defines the type of parent (e.g. 1 = app id or 1 = group id)
    pub like: u8,       // 1 byte - indicator if post has likes
    pub st: u8,         // 1 byte - status
    pub bump: u8,
}


#[account]
pub struct Chamber {
    pub ts: u32,
    pub uid: u32,
    pub sec: Vec<u8>,
    pub bump: u8,
}
