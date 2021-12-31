/*
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::sysvar;
use arrayref::array_ref;

use crate::{calculate_hash, MyErrorCode, HashOfHash};
use crate::state::data::Data;
use crate::state::house::House;

#[derive(Accounts)]
#[instruction(bet: u64)]
pub struct PullStrings<'info> {
    #[account(mut, seeds = [b"rng_house".as_ref(), & user.key().to_bytes(), & house.key().to_bytes(), & puppet.uuid.as_bytes()], bump = puppet.puppet_bump)]
    pub puppet: Account<'info, Data>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = sysvar::recent_blockhashes::id())]
    pub recent_blockhashes: AccountInfo<'info>,
    #[account(seeds = [b"rng_house".as_ref(), & house.author.to_bytes(), & house.operator.to_bytes()], bump = house.house_bump)]
    pub house: Account<'info, House>,
    #[account(mut, seeds = [b"rng_house".as_ref(), b"treasury".as_ref(), & house.key().to_bytes(), & house.author.to_bytes(), & house.operator.to_bytes()], bump = house.operator_treasury_bump)]
    pub operator_treasury: AccountInfo<'info>,

    #[account(mut, seeds = [b"rng_house".as_ref(), b"fees".as_ref(), & house.key().to_bytes(), & house.author.to_bytes(), & house.operator.to_bytes()], bump = house.author_fee_bump)]
    pub author_fee_account: AccountInfo<'info>,
    #[account(mut, seeds = [b"rng_house".as_ref(), b"fees".as_ref(), & house.key().to_bytes(), & house.author.to_bytes(), & house.operator.to_bytes()], bump = house.operator_fee_bump)]
    pub operator_fee_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PullStrings>, bet: u64) -> ProgramResult {
    let recent_blockhashes = &ctx.accounts.recent_blockhashes;
    let user = &ctx.accounts.user;
    if user.lamports() < bet {
        return Err(MyErrorCode::NotEnoughSOL.into());
    }
    let author_fee_account = &ctx.accounts.author_fee_account;

    let operator_fee_account = &ctx.accounts.operator_fee_account;

    let puppet = &mut ctx.accounts.puppet;

    let user_head = user.key;
    let fee1 = bet.checked_div(1000).ok_or(MyErrorCode::NumericalOverflowError)?
        .checked_mul(35).ok_or(MyErrorCode::NumericalOverflowError)?;
    let fee2 = bet.checked_div(1000).ok_or(MyErrorCode::NumericalOverflowError)?
        .checked_mul(2).ok_or(MyErrorCode::NumericalOverflowError)?;


    let data = recent_blockhashes.data.borrow();
    let most_recent = array_ref![data, 8, 8];
    let index = calculate_hash(&HashOfHash {
        recent_blockhash: *most_recent,
        user: user_head.to_bytes(),
    });

    puppet.data = index;
    invoke(
        &system_instruction::transfer(&user.key(), &ctx.accounts.operator_fee_account.key(), fee1),
        &[
            user.to_account_info().clone(),
            ctx.accounts.operator_fee_account.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],
    )?;
    invoke(
        &system_instruction::transfer(&user.key(), &ctx.accounts.author_fee_account.key(), fee2),
        &[
            user.to_account_info().clone(),
            ctx.accounts.author_fee_account.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],
    )?;
    invoke(
        &system_instruction::transfer(&user.key(), &ctx.accounts.operator_treasury.key(), bet),
        &[
            user.to_account_info().clone(),
            ctx.accounts.operator_treasury.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],
    )?;
    puppet.bet = bet;
    Ok(())
}


 */
