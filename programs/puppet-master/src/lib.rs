// #region core
mod utils;
mod constants;
mod errors;
mod myaccounts;
use std::ptr;
use std::convert::TryInto;
use anchor_lang::solana_program::sysvar;
use anchor_lang::solana_program::hash::Hash;
use anchor_lang::solana_program::program::invoke_signed;
use myaccounts::{House};
use anchor_lang::prelude::AnchorDeserialize;
use constants::{HOUSE_SIZE};
use errors::{ErrorCode};
use crate::utils::*;
use crate::utils::calculate_hash;
use anchor_lang::solana_program::sysvar::recent_blockhashes::*;
use anchor_lang::solana_program::sysvar::recent_blockhashes;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use anchor_spl::token::Token;
use arrayref::array_ref;
use core::ops::Deref;
use anchor_spl::token::Mint;
use puppet::cpi::accounts::SetData;
use puppet::program::Puppet;
use puppet::{self, Data};
use std::convert::AsMut;

declare_id!("9pJ55KszBGk1Td3LbRrWLszAaiXg7YLW5oouLABJwsZg");
const TREASURY: &str = "treasury";
const PREFIX: &str = "rng_house";
const FEES: &str = "fees";
const MINT: &str = "mint";
const SIGNER: &str = "signer";

#[program]
mod puppet_master {
    use super::*;
    pub fn initialize_operator_treasury(ctx: Context<InitializeOperatorTreasury>) -> ProgramResult {
        let operator_treasury_info = &mut ctx.accounts.operator_treasury;
        let payer = &ctx.accounts.payer;
        let treasury_info = &ctx.accounts.operator_treasury;
        
        let mut OT = OperatorTreasury {
            operator: *ctx.accounts.operator.key,
        };


        Ok(())
    }
    pub fn withdraw_funds<'info>(ctx: Context<WithdrawFunds<'info>>) -> ProgramResult {
        let operator = &ctx.accounts.operator;
        let pay = &ctx.accounts.operator_treasury.to_account_info();
        let snapshot: u64 = pay.lamports();

        **pay.lamports.borrow_mut() = 0;

        **operator.lamports.borrow_mut() = operator
            .lamports()
            .checked_add(snapshot)
            .ok_or(ErrorCode::NumericalOverflowError)?;

        Ok(())
    }

    pub fn create_house<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateHouse<'info>>,
        house_bump: u8,
        author_fee_bump: u8,
        operator_treasury_bump: u8,
        operator_fee_bump: u8,
        fee_basis_points: u16
    ) -> ProgramResult {
        if fee_basis_points > 10000 {
            return Err(errors::ErrorCode::InvalidBasisPoints.into());
        }

        let house = &mut ctx.accounts.house;
        let author = &mut ctx.accounts.author;
        
        let operator = &mut ctx.accounts.operator;
        
        
        let housekey = house.key();
        house.house_bump = house_bump;
        house.author_fee_bump = author_fee_bump;
        house.operator_fee_bump = operator_fee_bump;
        house.operator_treasury_bump = operator_treasury_bump;
        house.author = ctx.accounts.author.key();
        house.operator = ctx.accounts.operator.key();
        house.author_fee_account = ctx.accounts.author_fee_account.key();
        house.author_fee_account_destination = ctx.accounts.author_fee_account_destination.key();

        let author = &ctx.accounts.author;

        let treasury_seeds = &[PREFIX.as_bytes(), TREASURY.as_bytes(), housekey.as_ref(), house.author.as_ref(), house.operator.as_ref()];
        let treasury_info = &ctx.accounts.operator_treasury;
        house.operator_treasury = treasury_info.key();

        house.operator_treasury_destination = ctx.accounts.operator_treasury_destination.key();
        house.operator_fee_account = ctx.accounts.operator_fee_account.key();
        house.operator_fee_destination = ctx.accounts.operator_fee_destination.key();
        house.fee_basis_points = fee_basis_points;
        Ok(())
    }

    
    pub fn pull_strings(ctx: Context<PullStrings>, bet: u64, house_bump: u8, operator_fee_bump: u8, author_fee_bump: u8, operator_treasury_bump: u8    ) -> ProgramResult {
        let cpi_program = ctx.accounts.puppet_program.to_account_info();
        let operator_treasury = &ctx.accounts.operator_treasury;
        let user = &ctx.accounts.user;
        
        if user.lamports() < bet {
            return Err(ErrorCode::NotEnoughSOL.into());
        }
        let operator_fee_account = &ctx.accounts.operator_fee_account;
        let author_fee_account = &ctx.accounts.author_fee_account;
        let house = &ctx.accounts.house;
        let operator = &ctx.accounts.operator.to_account_info();
        let author = &ctx.accounts.author.to_account_info();
        let housekey = house.key().as_ref();
       // let recentHashes2 = RecentBlockhashes::default();
        let recentHashes = &ctx.accounts.recent_blockhashes;//RecentBlockhashes::deref(&recentHashes2);
        let data = recentHashes.data.borrow();
        /*
        let mut newHashes  = Vec::<[u8; 32]>::new();
        for hash in recentHashes {
            newHashes.push(hash.blockhash.to_bytes())
        }
        let mut newHashes2: [u8; 32] = newHashes.as_slice()[0];

    //    let mut newHashes2 = *newHashes.as_ref(); 
      //  newHashes2 : [u8; 32] = newHashes2;
        msg!("newhashes length {}", newHashes.len());
        */
        let pre_hellothere_recent2 = array_ref![data, 32, 32];

        let RNG : [u8; 32] = Hash::new_from_array(*pre_hellothere_recent2).to_bytes();//[4];
        let rngarray = array_ref![RNG, 8, 8];
        let mut  pre_hellothere_recent:  [u8; 8]  =  *array_ref![pre_hellothere_recent2, 8, 8];
        pre_hellothere_recent[0] = rngarray[0];

        let pre_index = calculate_hash(&HashOfHash {
            recent_blockhash: pre_hellothere_recent,
            user: user.key().to_bytes(),
        });
        
        
        // let hellothere_recent : [u8; 8] = array_ref![RNG, 8, 8];
        let house_acc = house.to_account_info();
        let FEE2 = (0.001544 as u64).checked_mul(bet as u64).ok_or(ErrorCode::NumericalOverflowError)?;
        let FEE = house.fee_basis_points.checked_div(10000).ok_or(ErrorCode::NumericalOverflowError)?.checked_mul(bet as u16).ok_or(ErrorCode::NumericalOverflowError)?;
        
        let index = calculate_hash(&HashOfHash {
            recent_blockhash: pre_hellothere_recent,
            user: user.key().to_bytes(),
        });
        invoke(
            &system_instruction::transfer(&user.key(), &author_fee_account.key(), FEE2 as u64),
            &[
                user.to_account_info().clone(),
                author_fee_account.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone(),
            ],
        )?;
        invoke(
            &system_instruction::transfer(&user.key(), &operator_fee_account.key(),  FEE as u64),
            &[
                user.to_account_info().clone(),
                operator_fee_account.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone(),
            ],
        )?;
        // let index = u64::from_le_bytes(*most_recent);
        let tos: String = index.to_string();
        let first: String = tos.chars().last().unwrap().to_string();
        let firstf: u64 = first.parse::<u64>().unwrap();
        let firstf2: f32 = first.parse::<f32>().unwrap();
        if firstf2 > 4.0 {
            let pay = &ctx.accounts.operator_treasury.to_account_info();
            let snapshot: u64 = pay.lamports();
    
            **pay.lamports.borrow_mut() = snapshot.checked_sub(bet.checked_mul(2).ok_or(ErrorCode::NumericalOverflowError)?).ok_or(ErrorCode::NumericalOverflowError)?;
    
            **user.lamports.borrow_mut() = user
                .lamports()
                .checked_add(bet.checked_mul(2).ok_or(ErrorCode::NumericalOverflowError)?).ok_or(ErrorCode::NumericalOverflowError)?;
    
    
    
            
            /*
            
        let signer_seeds = &[PREFIX.as_bytes(), TREASURY.as_bytes(), &*ctx.accounts.house.to_account_info().key.as_ref(),  house.author.as_ref(), house.operator.as_ref(),
        &[operator_treasury_bump],
    ];
        let accounts = &[operator_treasury.to_account_info().clone(),house.to_account_info().clone(), ctx.accounts.system_program.to_account_info().clone()];

        invoke_signed(
            &system_instruction::transfer(
               &operator_treasury.to_account_info().key,
                user.to_account_info().key,
                bet.checked_mul(2).ok_or(ErrorCode::NumericalOverflowError)?,
            ),accounts,
            
            &[signer_seeds],
        )?;
        

            */
    
        }
        if firstf2 <= 4.0 {
            invoke(
                &system_instruction::transfer(&user.key(), &operator_treasury.key(), bet),
                &[
                    user.to_account_info().clone(),
                    operator_treasury.to_account_info().clone(),
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

#[account]
#[derive(Default)]
pub struct OperatorTreasury {
    pub operator: Pubkey,
    // there's a borsh vec u32 denoting how many actual lines of data there are currently (eventually equals max number of lines)
    // There is actually lines and lines of data after this but we explicitly never want them deserialized.
    // here there is a borsh vec u32 indicating number of bytes in bitmask array.
    // here there is a number of bytes equal to ceil(max_number_of_lines/8) and it is a bit mask used to figure out when to increment borsh vec u32
}

#[derive(Accounts)]
#[instruction(bet: u64, house_bump: u8, operator_fee_bump: u8, author_fee_bump: u8, operator_treasury_bump: u8)]

pub struct PullStrings<'info> {
    author: UncheckedAccount<'info>,
    #[account(mut)]
    operator: UncheckedAccount<'info>,
    #[account(mut)]
    pub puppet: Account<'info, Data>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds=[PREFIX.as_bytes(), author.key().as_ref(), operator.key().as_ref()], bump=house_bump)]
    house: Account<'info, House>,
    #[account(mut, address = house.author_fee_account ) ]
    author_fee_account: UncheckedAccount<'info>,
    #[account(mut, address = house.operator_fee_account)]
    operator_fee_account: UncheckedAccount<'info>,
    
    #[account(address = sysvar::recent_blockhashes::id())]
    recent_blockhashes: UncheckedAccount<'info>,
    #[account(mut, address = house.operator_treasury)]
    operator_treasury: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub puppet_program: Program<'info, Puppet>,
}
#[derive(Accounts)]
pub struct InitializeOperatorTreasury<'info> {
    #[account(mut)]
    operator_treasury:AccountInfo<'info>,
    operator: AccountInfo<'info>,
    #[account(mut, signer)]
    payer: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(mut, has_one = operator)]
    operator_treasury:Account<'info, OperatorTreasury>,
    #[account(signer, address = operator_treasury.operator)]
    operator: AccountInfo<'info>,
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
    #[account(mut, seeds=[PREFIX.as_bytes(), FEES.as_bytes(), house.key().as_ref(), author.key.as_ref(), operator.key.as_ref()], bump=operator_fee_bump)]
    operator_fee_account: UncheckedAccount<'info>,
    author_fee_account_destination: UncheckedAccount<'info>,
    
    #[account(has_one=operator)]
    operator_treasury: ProgramAccount<'info, OperatorTreasury>,

    operator_treasury_destination: UncheckedAccount<'info>,


    operator_fee_destination: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}
