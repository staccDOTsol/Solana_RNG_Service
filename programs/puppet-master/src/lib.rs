// #region core
mod utils;
mod constants;
mod instructions;
mod state;
mod myerrors;

use crate::myerrors::myerrors::ErrorCode;
use crate::constants::constants::HOUSE_SIZE;
use crate::instructions::uncover::Uncover;
use crate::instructions::create_house::CreateHouse;
use crate::instructions::author_fee_withdraw::AuthorFeeWithdraw;
use crate::instructions::initialize::Initialize;
use crate::instructions::operator_fee_withdraw::OperatorFeeWithdraw;
use crate::instructions::operator_treasury_withdraw::OperatorTreasuryWithdraw;
use crate::instructions::pull_strings::PullStrings;
use crate::utils::utils::{calculate_hash, HashOfHash};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::sysvar;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use arrayref::array_ref;


declare_id!("9pJ55KszBGk1Td3LbRrWLszAaiXg7YLW5oouLABJwsZg");


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
            return Err(ErrorCode::InvalidBasisPoints.into());
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

    pub fn pull_strings(ctx: Context<PullStrings>, bet: u64) -> ProgramResult {
        let recent_blockhashes = &ctx.accounts.recent_blockhashes;
        let user = &ctx.accounts.user;
        if user.lamports() < bet {
            return Err(ErrorCode::NotEnoughSOL.into());
        }
        let author_fee_account = &ctx.accounts.author_fee_account;

        let operator_fee_account = &ctx.accounts.operator_fee_account;

        let puppet = &mut ctx.accounts.puppet;

        let user_head = user.key;
        let fee1 = bet.checked_div(1000).ok_or(ErrorCode::NumericalOverflowError)?
            .checked_mul(35).ok_or(ErrorCode::NumericalOverflowError)?;
        let fee2 = bet.checked_div(1000).ok_or(ErrorCode::NumericalOverflowError)?
            .checked_mul(2).ok_or(ErrorCode::NumericalOverflowError)?;


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
            &system_instruction::transfer(&user.key(),  &ctx.accounts.operator_treasury.key(),bet),
            &[
                user.to_account_info().clone(),
                ctx.accounts.operator_treasury.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone(),
            ],
        )?;
        // let index = u64::from_le_bytes(*most_recent);

        /*
        let cpi_accounts = SetData {
            puppet: ctx.accounts.puppet.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.puppet_program.to_account_info(), cpi_accounts);
        puppet::cpi::set_data(cpi_ctx, index)
        */
        puppet.bet = bet;
        Ok(())

    }
    pub fn uncover(ctx: Context<Uncover>) -> ProgramResult {
        let index = &ctx.accounts.puppet.data.clone();




        let first = index.to_string().chars().nth(0 as usize).unwrap().to_string();
        let firstf2: f32 = first.parse::<f32>().unwrap();



        let bet = ctx.accounts.puppet.bet;
        let house = &ctx.accounts.house;



        let user = &ctx.accounts.user;

        if firstf2 > 4.0 {
            invoke_signed(
                &system_instruction::transfer(&ctx.accounts.operator_treasury.key(), &user.key(), bet.checked_mul(2).ok_or(ErrorCode::NumericalOverflowError)?),
                &[
                    ctx.accounts.operator_treasury.to_account_info().clone(),
                    user.to_account_info().clone(),
                    ctx.accounts.system_program.to_account_info().clone(),
                ],//, &house.key().to_bytes(), author.key.as_ref(), operator.key.as_ref()
                &[&["rng_house".as_bytes(), "treasury".as_bytes(), &ctx.accounts.house.key().to_bytes(), &house.author.to_bytes(), &house.operator.to_bytes(), &[house.operator_treasury_bump]]],
            )?;
            Ok(())
        }
        else{
            return Err(ErrorCode::Lost.into());

        }
    }
    pub fn initialize(ctx: Context<Initialize>, puppet_bump: u8, uuid: String) -> ProgramResult {
        /*
        let signer_seeds = &[b"rng_house".as_ref(), &ctx.accounts.user.key().to_bytes(), &ctx.accounts.house.key().to_bytes(),
                &[puppet_bump],
            ];
        let accounts = &[puppet.to_account_info().clone(), ctx.accounts.system_program.to_account_info().clone()];

        invoke_signed(
            &system_instruction::assign(&puppet.key(), &ctx.program_id),
            accounts,
            &[signer_seeds],
        )?;
        */
        let puppet = &mut ctx.accounts.puppet;

        puppet.user = ctx.accounts.user.key();
        puppet.puppet_bump = puppet_bump;
        puppet.uuid = uuid;
        let recent_blockhashes = &ctx.accounts.recent_blockhashes;
        let data = recent_blockhashes.data.borrow();
        let most_recent = array_ref![data, 8, 8];
        let index = calculate_hash(&HashOfHash {
            recent_blockhash: *most_recent,
            user: puppet.user.to_bytes(),
        });

        puppet.data = index;
        Ok(())
    }
    pub fn author_fee_withdraw(ctx: Context<AuthorFeeWithdraw>, sol: u64) -> ProgramResult {
        /*
      invoke_signed(
                &system_instruction::transfer(&ctx.accounts.operator_treasury.key(), &user.key(), bet.checked_mul(2).ok_or(ErrorCode::NumericalOverflowError)?),
                &[
                    ctx.accounts.operator_treasury.to_account_info().clone(),
                    user.to_account_info().clone(),
                    ctx.accounts.system_program.to_account_info().clone(),
                ],//, &house.key().to_bytes(), author.key.as_ref(), operator.key.as_ref()
                &[&["rng_house".as_bytes(), "treasury".as_bytes(), &ctx.accounts.house.key().to_bytes(), &house.author.to_bytes(), &house.operator.to_bytes(), &[house.operator_treasury_bump]]],
            )?;
        */
        let author = &ctx.accounts.author;
        let house = &ctx.accounts.house;
        let author_fee_account = &ctx.accounts.author_fee_account;
        let author_fee_account_destination = &ctx.accounts.author_fee_account_destination;

        if sol > author_fee_account.lamports() {

            return Err(ErrorCode::NotEnoughSOL.into());
        }
        invoke_signed(
            &system_instruction::transfer(&author_fee_account.key(), &author_fee_account_destination.key(), sol),
            &[
                author_fee_account.to_account_info().clone(),
                author_fee_account_destination.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone(),
            ],//, &house.key().to_bytes(), author.key.as_ref(), operator.key.as_ref()
            &[&["rng_house".as_bytes(), "fees".as_bytes(), &house.key().to_bytes(), &house.author.to_bytes(), &house.operator.to_bytes(), &[house.author_fee_bump]]],
        )?;
        Ok(())
    }


    pub fn operator_fee_withdraw(ctx: Context<OperatorFeeWithdraw>, sol: u64) -> ProgramResult {
        /*
      invoke_signed(
                &system_instruction::transfer(&ctx.accounts.operator_treasury.key(), &user.key(), bet.checked_mul(2).ok_or(ErrorCode::NumericalOverflowError)?),
                &[
                    ctx.accounts.operator_treasury.to_account_info().clone(),
                    user.to_account_info().clone(),
                    ctx.accounts.system_program.to_account_info().clone(),
                ],//, &house.key().to_bytes(), author.key.as_ref(), operator.key.as_ref()
                &[&["rng_house".as_bytes(), "treasury".as_bytes(), &ctx.accounts.house.key().to_bytes(), &house.author.to_bytes(), &house.operator.to_bytes(), &[house.operator_treasury_bump]]],
            )?;
        */
        let operator = &ctx.accounts.operator;
        let house = &ctx.accounts.house;
        let operator_fee_account = &ctx.accounts.operator_fee_account;
        let operator_fee_destination = &ctx.accounts.operator_fee_destination;

        if sol > operator_fee_account.lamports() {

            return Err(ErrorCode::NotEnoughSOL.into());
        }
        invoke_signed(
            &system_instruction::transfer(&operator_fee_account.key(), &operator_fee_destination.key(), sol),
            &[
                operator_fee_account.to_account_info().clone(),
                operator_fee_destination.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone(),
            ],//, &house.key().to_bytes(), author.key.as_ref(), operator.key.as_ref()
            &[&["rng_house".as_bytes(), "fees".as_bytes(), &house.key().to_bytes(), &house.author.to_bytes(), &house.operator.to_bytes(), &[house.operator_fee_bump]]],
        )?;
        Ok(())
    }

    pub fn operator_treasury_withdraw(ctx: Context<OperatorTreasuryWithdraw>, sol: u64) -> ProgramResult {
        /*
     invoke_signed(
               &system_instruction::transfer(&ctx.accounts.operator_treasury.key(), &user.key(), bet.checked_mul(2).ok_or(ErrorCode::NumericalOverflowError)?),
               &[
                   ctx.accounts.operator_treasury.to_account_info().clone(),
                   user.to_account_info().clone(),
                   ctx.accounts.system_program.to_account_info().clone(),
               ],//, &house.key().to_bytes(), author.key.as_ref(), operator.key.as_ref()
               &[&["rng_house".as_bytes(), "treasury".as_bytes(), &ctx.accounts.house.key().to_bytes(), &house.author.to_bytes(), &house.operator.to_bytes(), &[house.operator_treasury_bump]]],
           )?;
       */
        let operator = &ctx.accounts.operator;
        let house = &ctx.accounts.house;
        let operator_treasury = &ctx.accounts.operator_treasury;
        let operator_treasury_destination = &ctx.accounts.operator_treasury_destination;

        if sol > operator_treasury.lamports() {

            return Err(ErrorCode::NotEnoughSOL.into());
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
}
