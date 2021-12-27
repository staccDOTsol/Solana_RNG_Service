// #region core
pub mod utils;
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::sysvar;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use arrayref::array_ref;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

declare_id!("EqP43dPi9EWyqBEm543a8QwZQV5WamWMDyCi7vousBuM");
const TREASURY: &str = "treasury";
const PREFIX: &str = "rng_house";
const FEE_PAYER: &str = "fee_payer";
const SIGNER: &str = "signer";
const FEE: u64 = 5000000;

pub const HOUSE_SIZE: usize = 8 + //key
    32 + // fee payer
    32 + // treasury
    32 + // treasury_withdrawal_destination
    32 + // fee withdrawal destination
    32 + // treasury mint
    32 + // house_author
    32 + // house_operator
    1  + // bump
    1  + // treasury_bump
    1  + // fee_payer_bump
    2  + // fee basis points
    220; // padding

#[program]
mod puppet_master {
    use super::*;

    pub fn create_house<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateHouse<'info>>,
        bump: u8,
        fee_payer_bump: u8,
        treasury_bump: u8,
        fee_basis_points: u16,
    ) -> ProgramResult {
        let house_author = &ctx.accounts.house_author;
        let house = &mut ctx.accounts.house;
        let house_fee_account = &ctx.accounts.house_fee_account;
        let house_operator = &ctx.accounts.house_operator;
        let house_treasury = &ctx.accounts.house_treasury;
        let fee_withdrawal_destination = &ctx.accounts.fee_withdrawal_destination;
        let treasury_withdrawal_destination_owner =
            &ctx.accounts.treasury_withdrawal_destination_owner;
        let treasury_withdrawal_destination = &ctx.accounts.treasury_withdrawal_destination;

        house.bump = bump;
        house.fee_payer_bump = fee_payer_bump;
        house.treasury_bump = treasury_bump;
        if fee_basis_points > 10000 {
            return Err(ErrorCode::InvalidBasisPoints.into());
        }
        house.fee_basis_points = fee_basis_points;
        house.house_operator = house_operator.key();
        house.house_author = house_author.key();
        house.house_fee_account = house_fee_account.key();
        house.house_treasury = house_treasury.key();
        house.treasury_withdrawal_destination = treasury_withdrawal_destination.key();
        house.fee_withdrawal_destination = fee_withdrawal_destination.key();

        Ok(())
    }

    // TODO - https://docs.solana.com/developing/runtime-facilities/sysvars

    /*
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
     */
}

#[derive(Accounts)]
#[instruction(bump: u8, fee_payer_bump: u8, treasury_bump: u8, fee_basis_points: u16)]
pub struct CreateHouse<'info> {
    payer: Signer<'info>,
    house_author: UncheckedAccount<'info>,
    house_operator: UncheckedAccount<'info>,
    #[account(mut)]
    fee_withdrawal_destination: UncheckedAccount<'info>,
    #[account(mut)]
    treasury_withdrawal_destination: UncheckedAccount<'info>,
    #[account(mut)]
    treasury_withdrawal_destination_owner: UncheckedAccount<'info>,
    #[account(init, seeds=[PREFIX.as_bytes(), house_author.key().as_ref(), house_operator.key().as_ref()], bump=bump, space=HOUSE_SIZE, payer=payer)]
    house: Account<'info, House>,
    #[account(mut, seeds=[PREFIX.as_bytes(), house.key().as_ref(), FEE_PAYER.as_bytes()], bump=fee_payer_bump)]
    house_fee_account: UncheckedAccount<'info>,
    #[account(mut, seeds=[PREFIX.as_bytes(), house.key().as_ref(), TREASURY.as_bytes()], bump=treasury_bump)]
    house_treasury: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

/*
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
 */

// #endregion core

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(Hash)]
pub struct HashOfHash {
    pub recent_blockhash: [u8; 8],
    pub user: [u8; 32],
}

#[account]
pub struct House {
    pub house_fee_account: Pubkey,
    pub house_treasury: Pubkey,
    pub treasury_withdrawal_destination: Pubkey,
    pub fee_withdrawal_destination: Pubkey,
    pub house_author: Pubkey,
    pub house_operator: Pubkey,
    pub bump: u8,
    pub treasury_bump: u8,
    pub fee_payer_bump: u8,
    pub fee_basis_points: u16,
}

#[error]
pub enum ErrorCode {
    #[msg("Not enough SOL to pay for this minting")]
    NotEnoughSOL,

    #[msg("Numerical overflow error!")]
    NumericalOverflowError,

    #[msg("Unable to find an unused config line near your random number index")]
    CannotFindUsableConfigLine,

    #[msg("BP must be less than or equal to 10000")]
    InvalidBasisPoints,

    #[msg("PublicKeyMismatch")]
    PublicKeyMismatch,

    #[msg("UninitializedAccount")]
    UninitializedAccount,

    #[msg("IncorrectOwner")]
    IncorrectOwner,
}
