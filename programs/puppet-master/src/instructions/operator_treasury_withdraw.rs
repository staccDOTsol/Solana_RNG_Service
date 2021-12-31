use anchor_lang::prelude::*;
use crate::state::house::House;

#[derive(Accounts)]
pub struct OperatorTreasuryWithdraw<'info> {

    #[account(seeds=[b"rng_house".as_ref(), &house.author.to_bytes(), &house.operator.to_bytes()], bump=house.house_bump)]
    pub house: Account<'info, House>,

    #[account(address=house.operator_treasury)]
    pub operator_treasury: AccountInfo<'info>,
    #[account(address=house.operator_treasury_destination)]
    pub operator_treasury_destination: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    #[account(address=house.operator)]
    pub operator: Signer<'info>,
}
