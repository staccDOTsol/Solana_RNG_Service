use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use crate::state::data::Data;
use crate::state::house::House;
use crate::constants::constants::HOUSE_SIZE;

#[derive(Accounts)]
#[instruction(house_bump: u8, author_fee_bump: u8, operator_treasury_bump: u8, operator_fee_bump: u8, fee_basis_points: u16)]
pub struct CreateHouse<'info> {
    pub author: Signer<'info>,
    #[account(mut)]
    pub operator: AccountInfo<'info>,
    #[account(init, seeds=[b"rng_house".as_ref(), &author.key().to_bytes(), &operator.key.to_bytes()], bump=house_bump, space=HOUSE_SIZE, payer=author)]
    pub house: Account<'info, House>,
    #[account(mut, seeds=[b"rng_house".as_ref(), b"fees".as_ref(), &house.key().to_bytes(), &author.key.to_bytes(), &operator.key.to_bytes()], bump=author_fee_bump)]
    pub author_fee_account: AccountInfo<'info>,
    pub author_fee_account_destination: AccountInfo<'info>,
    //#[account( seeds = [b"house_treasury".as_ref(), &initializer.key.to_bytes(), &authority.key.to_bytes()],

    #[account(mut, seeds=[b"rng_house".as_ref(), b"treasury".as_ref(), &house.key().to_bytes(), &author.key.to_bytes(), &operator.key.to_bytes()], bump=operator_treasury_bump)]
    pub operator_treasury: AccountInfo<'info>,
    pub operator_treasury_destination: AccountInfo<'info>,
    #[account(mut, seeds=[b"rng_house".as_ref(), b"fees".as_ref(), &house.key().to_bytes(), &author.key.to_bytes(), &operator.key.to_bytes()], bump=operator_fee_bump)]
    pub operator_fee_account: AccountInfo<'info>,
    pub operator_fee_destination: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
