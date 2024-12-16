use anchor_lang::prelude::*;
//use anchor_spl::token::Token;

#[account]
pub struct House {
    
    pub author_fee_account: Pubkey,
    pub author_fee_account_destination: Pubkey,
    pub operator_treasury: Pubkey,
    pub operator_treasury_destination: Pubkey,
    pub operator_fee_account: Pubkey,
    pub operator_fee_destination: Pubkey,
    pub author: Pubkey,
    pub operator: Pubkey,
    pub token_mint: Pubkey,
    pub house_bump: u8,
    pub author_fee_bump: u8,
    pub token_mint_bump: u8,
    pub operator_treasury_bump: u8,
    pub operator_fee_bump: u8,
    pub fee_basis_points: u16,
}
