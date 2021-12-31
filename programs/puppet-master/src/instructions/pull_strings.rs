use anchor_lang::prelude::*;
use crate::state::data::Data;
use crate::state::house::House;
use anchor_lang::solana_program::sysvar;

#[derive(Accounts)]
#[instruction(bet: u64)]
pub struct PullStrings<'info> {

    #[account(mut,seeds=[b"rng_house".as_ref(), &user.key().to_bytes(), &house.key().to_bytes(), &puppet.uuid.as_bytes()],  bump=puppet.puppet_bump)]
    pub puppet: Account<'info, Data>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = sysvar::recent_blockhashes::id())]
    pub recent_blockhashes: AccountInfo<'info>,
    #[account(seeds=[b"rng_house".as_ref(), &house.author.to_bytes(), &house.operator.to_bytes()], bump=house.house_bump)]
    pub house: Account<'info, House>,
    #[account(mut, seeds=[b"rng_house".as_ref(), b"treasury".as_ref(), &house.key().to_bytes(), &house.author.to_bytes(), &house.operator.to_bytes()], bump=house.operator_treasury_bump)]
    pub operator_treasury: AccountInfo<'info>,

    #[account(mut, seeds=[b"rng_house".as_ref(), b"fees".as_ref(), &house.key().to_bytes(), &house.author.to_bytes(), &house.operator.to_bytes()], bump=house.author_fee_bump)]
    pub author_fee_account: AccountInfo<'info>,
    #[account(mut, seeds=[b"rng_house".as_ref(), b"fees".as_ref(), &house.key().to_bytes(), &house.author.to_bytes(), &house.operator.to_bytes()], bump=house.operator_fee_bump)]
    pub operator_fee_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
