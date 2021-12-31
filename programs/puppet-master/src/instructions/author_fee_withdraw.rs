use anchor_lang::prelude::*;
use crate::state::data::Data;
use crate::state::house::House;


#[derive(Accounts)]
pub struct AuthorFeeWithdraw<'info> {

    #[account(seeds=[b"rng_house".as_ref(), &house.author.to_bytes(), &house.operator.to_bytes()], bump=house.house_bump)]
    pub house: Account<'info, House>,

    #[account(address=house.author_fee_account)]
    pub author_fee_account: AccountInfo<'info>,
    #[account(address=house.author_fee_account_destination)]
    pub author_fee_account_destination: AccountInfo<'info>,
    #[account(address=house.author)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}
