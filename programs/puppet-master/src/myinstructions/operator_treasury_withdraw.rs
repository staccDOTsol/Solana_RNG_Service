use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::sysvar;

use crate::state::*;

#[repr(C)]
#[derive(Accounts)]
pub struct OperatorTreasuryWithdraw<'info> {
    #[account(seeds = [b"rng_house".as_ref(), & house.author.to_bytes(), & house.operator.to_bytes()], bump = house.house_bump)]
    pub house: Account<'info, House>,

    #[account(address = house.operator_treasury)]
    pub operator_treasury: AccountInfo<'info>,
    #[account(address = house.operator_treasury_destination)]
    pub operator_treasury_destination: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    #[account(address = house.operator)]
    pub operator: Signer<'info>,
}


pub fn handler(ctx: Context<OperatorTreasuryWithdraw>, sol: u64) -> ProgramResult {
    let operator = &ctx.accounts.operator;
    let house = &ctx.accounts.house;
    let operator_treasury = &ctx.accounts.operator_treasury;
    let operator_treasury_destination = &ctx.accounts.operator_treasury_destination;

    if sol > operator_treasury.lamports() {
        return Err(MyErrorCode::NotEnoughSOL.into());
    }
    invoke_signed(
        &system_instruction::transfer(&operator_treasury.key(), &operator_treasury_destination.key(), sol),
        &[
            operator_treasury.to_account_info().clone(),
            operator_treasury_destination.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],//, &house.key().to_bytes(), author.key.as_ref(), operator.key.as_ref()
        &[&["rng_house".as_bytes(), "treasury".as_bytes(), &house.key().to_bytes(), &house.author.to_bytes(), &house.operator.to_bytes(), &[house.operator_treasury_bump]]],
    )?;
    Ok(())
}
