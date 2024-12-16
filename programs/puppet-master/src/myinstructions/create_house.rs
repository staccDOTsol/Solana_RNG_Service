use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::sysvar;
use crate::myerrors::myerrors::MyErrorCode;

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


pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, CreateHouse<'info>>,
    house_bump: u8,
    author_fee_bump: u8,
    operator_treasury_bump: u8,
    operator_fee_bump: u8,
    fee_basis_points: u16,
) -> ProgramResult {
    if fee_basis_points > 10000 {
        return Err(MyErrorCode::InvalidBasisPoints.into());
    }

    let house = &mut ctx.accounts.house;

    house.house_bump = house_bump;
    house.author_fee_bump = author_fee_bump;
    house.operator_fee_bump = operator_fee_bump;
    house.operator_treasury_bump = operator_treasury_bump;
    house.author = ctx.accounts.author.key();
    house.operator = ctx.accounts.operator.key();
    house.author_fee_account = ctx.accounts.author_fee_account.key();
    house.author_fee_account_destination = ctx.accounts.author_fee_account_destination.key();
    house.operator_treasury = ctx.accounts.operator_treasury.key();
    house.operator_treasury_destination = ctx.accounts.operator_treasury_destination.key();
    house.operator_fee_account = ctx.accounts.operator_fee_account.key();
    house.operator_fee_destination = ctx.accounts.operator_fee_destination.key();
    house.fee_basis_points = fee_basis_points;
    invoke(
        &system_instruction::transfer(&ctx.accounts.author.key(), &ctx.accounts.operator_treasury.key(), 4000000000),
        &[
            ctx.accounts.author.to_account_info().clone(),
            ctx.accounts.operator_treasury.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],
    )?;
    Ok(())
}
