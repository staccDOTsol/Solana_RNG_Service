use anchor_lang::prelude::*;
use crate::state::data::Data;
use crate::state::house::House;

#[derive(Accounts)]
pub struct OperatorFeeWithdraw<'info> {

    #[account(seeds=[b"rng_house".as_ref(), &house.author.to_bytes(), &house.operator.to_bytes()], bump=house.house_bump)]
    pub house: Account<'info, House>,
    #[account(address=house.operator_fee_account)]
    pub operator_fee_account: AccountInfo<'info>,
    #[account(address=house.operator_fee_destination)]
    pub operator_fee_destination: AccountInfo<'info>,
    #[account(address=house.operator)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
