use anchor_lang::prelude::*;

#[account]
pub struct Data {
    pub puppet_bump: u8,
    pub data: u64,
    pub user: Pubkey,
    pub bet: u64,
    pub uuid: String,
}
