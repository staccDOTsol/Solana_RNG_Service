/*
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::sysvar;

#[repr(C)]
#[derive(Accounts)]
pub struct Uncover<'info> {
    #[account(mut, seeds = [b"rng_house".as_ref(), & user.key().to_bytes(), & house.key().to_bytes(), & puppet.uuid.as_bytes()], bump = puppet.puppet_bump)]
    pub puppet: Account<'info, Data>,
    #[account(address = puppet.user)]
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

pub fn handler(ctx: Context<Uncover>) -> ProgramResult {
    let index = &ctx.accounts.puppet.data.clone();


    let first = index.to_string().chars().nth(0 as usize).unwrap().to_string();
    let firstf2: f32 = first.parse::<f32>().unwrap();


    let bet = ctx.accounts.puppet.bet;
    let house = &ctx.accounts.house;

    let user = &ctx.accounts.user;

    if firstf2 > 4.0 {
        invoke_signed(
            &system_instruction::transfer(&ctx.accounts.operator_treasury.key(), &user.key(), bet.checked_mul(2).ok_or(MyErrorCode::NumericalOverflowError)?),
            &[
                ctx.accounts.operator_treasury.to_account_info().clone(),
                user.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone(),
            ],//, &house.key().to_bytes(), author.key.as_ref(), operator.key.as_ref()
            &[&["rng_house".as_bytes(), "treasury".as_bytes(), &ctx.accounts.house.key().to_bytes(), &house.author.to_bytes(), &house.operator.to_bytes(), &[house.operator_treasury_bump]]],
        )?;
        Ok(())
    } else {
        return Err(MyErrorCode::Lost.into());
    }
}


 */
