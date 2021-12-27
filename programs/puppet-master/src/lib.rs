// #region core
mod utils;
mod constants;
mod errors;
mod myaccounts;

use myaccounts::{House};
use constants::{HOUSE_SIZE};
use errors::{ErrorCode};
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::sysvar;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use arrayref::array_ref;


declare_id!("EqP43dPi9EWyqBEm543a8QwZQV5WamWMDyCi7vousBuM");
const TREASURY: &str = "treasury";
const PREFIX: &str = "rng_house";
const FEES: &str = "fees";
const SIGNER: &str = "signer";
const FEE: u64 = 5000000;

#[program]
mod puppet_master {
    use super::*;

    pub fn create_house<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateHouse<'info>>,
        house_bump: u8,
        author_fee_bump: u8,
        operator_treasury_bump: u8,
        operator_fee_bump: u8,
        fee_basis_points: u16,
    ) -> ProgramResult {
        if fee_basis_points > 10000 {
            return Err(errors::ErrorCode::InvalidBasisPoints.into());
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
        Ok(())
    }

    pub fn pull_strings(ctx: Context<PullStrings>, bet: u64) -> ProgramResult {
        let cpi_program = ctx.accounts.puppet_program.to_account_info();
        let recent_blockhashes = &ctx.accounts.recent_blockhashes;
        let user = &ctx.accounts.user;
        if user.lamports() < bet {
            return Err(ErrorCode::NotEnoughSOL.into());
        }
        let house_fee_account = &ctx.accounts.house_fee_account;
        let house = &ctx.accounts.house;
        let data = recent_blockhashes.data.borrow();
        let most_recent = array_ref![data, 8, 8];
        let user_head = user.key;
        let index = calculate_hash(&HashOfHash {
            recent_blockhash: *most_recent,
            user: user_head.to_bytes(),
        });
        invoke(
            &system_instruction::transfer(user.key, house_fee_account.key, FEE),
            &[
                user.to_account_info().clone(),
                house_fee_account.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone(),
            ],
        )?;
        // let index = u64::from_le_bytes(*most_recent);
        let tos: String = index.to_string();
        let first: String = tos.chars().last().unwrap().to_string();
        let firstf: u64 = first.parse::<u64>().unwrap();
        let firstf2: f32 = first.parse::<f32>().unwrap();
        if firstf2 > 4.0 {
            invoke(
                &system_instruction::transfer(house_fee_account.key, user.key, bet * 1.85 as u64),
                &[
                    house_fee_account.to_account_info().clone(),
                    user.to_account_info().clone(),
                    ctx.accounts.system_program.to_account_info().clone(),
                ],
            )?;
        }
        if firstf2 <= 4.0 {
            invoke(
                &system_instruction::transfer(user.key, house_fee_account.key, bet),
                &[
                    user.to_account_info().clone(),
                    house_fee_account.to_account_info().clone(),
                    ctx.accounts.system_program.to_account_info().clone(),
                ],
            )?;
        }

        let cpi_accounts = SetData {
            puppet: ctx.accounts.puppet.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.puppet_program.to_account_info(), cpi_accounts);
        puppet::cpi::set_data(cpi_ctx, firstf)
    }
}

#[derive(Accounts)]
pub struct PullStrings<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Data>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = sysvar::recent_blockhashes::id())]
    recent_blockhashes: UncheckedAccount<'info>,
    #[account(seeds=[PREFIX.as_bytes(), house.house_operator.as_ref()], bump=house.bump, has_one=house_fee_account)]
    house: Account<'info, House>,
    #[account(mut, seeds=[PREFIX.as_bytes(), house.key().as_ref(), FEE_PAYER.as_bytes()], bump=house.fee_payer_bump)]
    house_fee_account: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub puppet_program: Program<'info, Puppet>,
}

// #endregion core

#[derive(Accounts)]
#[instruction(house_bump: u8, author_fee_bump: u8, operator_treasury_bump: u8, operator_fee_bump: u8, fee_basis_points: u16)]
pub struct CreateHouse<'info> {
    author: Signer<'info>,
    #[account(mut)]
    operator: UncheckedAccount<'info>,
    #[account(init, seeds=[PREFIX.as_bytes(), author.key().as_ref(), operator.key().as_ref()], bump=house_bump, space=HOUSE_SIZE, payer=author)]
    house: Account<'info, House>,
    #[account(mut, seeds=[PREFIX.as_bytes(), FEES.as_bytes(), house.key().as_ref(), author.key.as_ref(), operator.key.as_ref()], bump=author_fee_bump)]
    author_fee_account: UncheckedAccount<'info>,
    author_fee_account_destination: UncheckedAccount<'info>,
    #[account(mut, seeds=[PREFIX.as_bytes(), TREASURY.as_bytes(), house.key().as_ref(), author.key.as_ref(), operator.key.as_ref()], bump=operator_treasury_bump)]
    operator_treasury: UncheckedAccount<'info>,
    operator_treasury_destination: UncheckedAccount<'info>,
    #[account(mut, seeds=[PREFIX.as_bytes(), FEES.as_bytes(), house.key().as_ref(), author.key.as_ref(), operator.key.as_ref()], bump=operator_fee_bump)]
    operator_fee_account: UncheckedAccount<'info>,
    operator_fee_destination: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}
