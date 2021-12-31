// #region core
mod utils;
mod constants;
mod errors;
mod myaccounts;

use anchor_lang::solana_program::clock;
use myaccounts::{House};
use constants::{HOUSE_SIZE};
use errors::{ErrorCode};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::sysvar;
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
        let house = &ctx.accounts.house;
        if user.lamports() < bet {
            return Err(ErrorCode::NotEnoughSOL.into());
        }
        
        let puppet = &mut ctx.accounts.puppet;
        
        let user_head = user.key;
        let fee1 = bet.checked_div(10000).ok_or(ErrorCode::NumericalOverflowError)?
        .checked_mul(house.fee_basis_points as u64).ok_or(ErrorCode::NumericalOverflowError)?;
        let fee2 = bet.checked_div(10000).ok_or(ErrorCode::NumericalOverflowError)?
        .checked_mul((house.fee_basis_points  as u64).checked_div(15).ok_or(ErrorCode::NumericalOverflowError)?).ok_or(ErrorCode::NumericalOverflowError)?;

        
        let data = recent_blockhashes.data.borrow();
        let most_recent = array_ref![data, 8, 8];
        let clock = clock::Clock::get().unwrap().unix_timestamp as u8;
        let clock_arr: [u8; 1] = [clock];
        let index = calculate_hash(&HashOfHash {
            recent_blockhash: *most_recent,
            user: user_head.to_bytes(),
            clock: clock_arr
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
        let mut index = ctx.accounts.puppet.data.clone() as usize;
        index = index.checked_add(1000000000).ok_or(ErrorCode::NumericalOverflowError)?;
        while index > 1000000000 {
            let first = index.to_string().chars().nth(0).unwrap();

            let mut firstf2: u32 = first.to_string().parse::<u32>().unwrap();
            
            firstf2 = firstf2.checked_add(1 as u32).ok_or(ErrorCode::NumericalOverflowError)?;
            firstf2 = firstf2.checked_mul(10 as u32).ok_or(ErrorCode::NumericalOverflowError)?;
            let last = index.to_string().chars().nth(0).unwrap();
            let mut lastf2: u32 = last.to_string().parse::<u32>().unwrap();
            lastf2 = lastf2.checked_add(1 as u32).ok_or(ErrorCode::NumericalOverflowError)?;
            lastf2 = lastf2.checked_mul(10 as u32).ok_or(ErrorCode::NumericalOverflowError)?;
           index = index.checked_div(lastf2 as usize).ok_or(ErrorCode::NumericalOverflowError)?.checked_div(firstf2 as usize).ok_or(ErrorCode::NumericalOverflowError)?;
        }


        let bet = ctx.accounts.puppet.bet;
        let house = &ctx.accounts.house;
        


        let user = &ctx.accounts.user;
        ctx.accounts.puppet.data = 777;
        let first = index.to_string().chars().last().unwrap();

        let firstf2: f32 = first.to_string().parse::<f32>().unwrap();
    if firstf2 <= 4.0 {
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
        return Err(errors::ErrorCode::Lost.into());

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


#[derive(Accounts)]
pub struct Uncover<'info> {
    #[account(mut,seeds=[b"rng_house".as_ref(), &user.key().to_bytes(), &house.key().to_bytes(), &puppet.uuid.as_bytes()],  bump=puppet.puppet_bump)]
    pub puppet: Account<'info, Data>,
    #[account(address=puppet.user)]
    pub user: Signer<'info>,

    #[account(address = sysvar::recent_blockhashes::id())]
    recent_blockhashes: AccountInfo<'info>,
    #[account(seeds=[b"rng_house".as_ref(), &house.author.to_bytes(), &house.operator.to_bytes()], bump=house.house_bump)]
    house: Account<'info, House>,
    #[account(mut, seeds=[b"rng_house".as_ref(), b"treasury".as_ref(), &house.key().to_bytes(), &house.author.to_bytes(), &house.operator.to_bytes()], bump=house.operator_treasury_bump)]
    operator_treasury: AccountInfo<'info>,
    
    #[account(mut, seeds=[b"rng_house".as_ref(), b"fees".as_ref(), &house.key().to_bytes(), &house.author.to_bytes(), &house.operator.to_bytes()], bump=house.author_fee_bump)]
    author_fee_account: AccountInfo<'info>,
    #[account(mut, seeds=[b"rng_house".as_ref(), b"fees".as_ref(), &house.key().to_bytes(), &house.author.to_bytes(), &house.operator.to_bytes()], bump=house.operator_fee_bump)]
    operator_fee_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction(bet: u64)]
pub struct PullStrings<'info> {

    #[account(mut,seeds=[b"rng_house".as_ref(), &user.key().to_bytes(), &house.key().to_bytes(), &puppet.uuid.as_bytes()],  bump=puppet.puppet_bump)]
    pub puppet: Account<'info, Data>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = sysvar::recent_blockhashes::id())]
    recent_blockhashes: AccountInfo<'info>,
    #[account(seeds=[b"rng_house".as_ref(), &house.author.to_bytes(), &house.operator.to_bytes()], bump=house.house_bump)]
    house: Account<'info, House>,
    #[account(mut, seeds=[b"rng_house".as_ref(), b"treasury".as_ref(), &house.key().to_bytes(), &house.author.to_bytes(), &house.operator.to_bytes()], bump=house.operator_treasury_bump)]
    operator_treasury: AccountInfo<'info>,
    
    #[account(mut, seeds=[b"rng_house".as_ref(), b"fees".as_ref(), &house.key().to_bytes(), &house.author.to_bytes(), &house.operator.to_bytes()], bump=house.author_fee_bump)]
    author_fee_account: AccountInfo<'info>,
    #[account(mut, seeds=[b"rng_house".as_ref(), b"fees".as_ref(), &house.key().to_bytes(), &house.author.to_bytes(), &house.operator.to_bytes()], bump=house.operator_fee_bump)]
    operator_fee_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

// #endregion core

#[derive(Accounts)]
#[instruction(house_bump: u8, author_fee_bump: u8, operator_treasury_bump: u8, operator_fee_bump: u8, fee_basis_points: u16)]
pub struct CreateHouse<'info> {
    author: Signer<'info>,
    #[account(mut)]
    operator: AccountInfo<'info>,
    #[account(init, seeds=[b"rng_house".as_ref(), &author.key().to_bytes(), &operator.key.to_bytes()], bump=house_bump, space=HOUSE_SIZE, payer=author)]
    house: Account<'info, House>,
    #[account(mut, seeds=[b"rng_house".as_ref(), b"fees".as_ref(), &house.key().to_bytes(), &author.key.to_bytes(), &operator.key.to_bytes()], bump=author_fee_bump)]
    author_fee_account: AccountInfo<'info>,
    author_fee_account_destination: AccountInfo<'info>,
    //#[account( seeds = [b"house_treasury".as_ref(), &initializer.key.to_bytes(), &authority.key.to_bytes()],

    #[account(mut, seeds=[b"rng_house".as_ref(), b"treasury".as_ref(), &house.key().to_bytes(), &author.key.to_bytes(), &operator.key.to_bytes()], bump=operator_treasury_bump)]
    operator_treasury: AccountInfo<'info>,
    operator_treasury_destination: AccountInfo<'info>,
    #[account(mut, seeds=[b"rng_house".as_ref(), b"fees".as_ref(), &house.key().to_bytes(), &author.key.to_bytes(), &operator.key.to_bytes()], bump=operator_fee_bump)]
    operator_fee_account: AccountInfo<'info>,
    operator_fee_destination: AccountInfo<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(puppet_bump: u8, uuid: String)]
pub struct Initialize<'info> {
    // [Buffer.from("rng_house"), provider.wallet.publicKey.toBuffer(), houseObj.operator.toBuffer()]
    //init, seeds=[b"rng_house".as_ref(), &author.key().to_bytes(), &operator.key.to_bytes()], bump=house_bump, space=HOUSE_SIZE, payer=author)]
    #[account(init ,seeds=[b"rng_house".as_ref(), &user.key().to_bytes(), &house.key().to_bytes(), uuid.as_bytes()],  bump=puppet_bump, space=HOUSE_SIZE, payer=user)]
    pub puppet: Account<'info, Data>,
    pub user: Signer<'info>,

    #[account(address = sysvar::recent_blockhashes::id())]
    recent_blockhashes: AccountInfo<'info>,
    #[account(seeds=[b"rng_house".as_ref(), &house.author.to_bytes(), &house.operator.to_bytes()], bump=house.house_bump)]
    house: Account<'info, House>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OperatorTreasuryWithdraw<'info> {

    #[account(seeds=[b"rng_house".as_ref(), &house.author.to_bytes(), &house.operator.to_bytes()], bump=house.house_bump)]
    house: Account<'info, House>,
   
#[account(address=house.operator_treasury)]
operator_treasury: AccountInfo<'info>,
#[account(address=house.operator_treasury_destination)]
operator_treasury_destination: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,

    #[account(address=house.operator)]
        operator: Signer<'info>,
}



#[derive(Accounts)]
pub struct OperatorFeeWithdraw<'info> {

    #[account(seeds=[b"rng_house".as_ref(), &house.author.to_bytes(), &house.operator.to_bytes()], bump=house.house_bump)]
    house: Account<'info, House>,
#[account(address=house.operator_fee_account)]
operator_fee_account: AccountInfo<'info>,
#[account(address=house.operator_fee_destination)]
operator_fee_destination: AccountInfo<'info>,
#[account(address=house.operator)]
operator: Signer<'info>,
pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct AuthorFeeWithdraw<'info> {

    #[account(seeds=[b"rng_house".as_ref(), &house.author.to_bytes(), &house.operator.to_bytes()], bump=house.house_bump)]
    house: Account<'info, House>,

    #[account(address=house.author_fee_account)]
    author_fee_account: AccountInfo<'info>,
    #[account(address=house.author_fee_account_destination)]
    author_fee_account_destination: AccountInfo<'info>,
#[account(address=house.author)]
author: Signer<'info>,
pub system_program: Program<'info, System>,
}
#[account]
pub struct Data {
    puppet_bump: u8,
    data: u64,
    user: Pubkey,
    bet: u64,
    uuid: String,
}
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(Hash)]
 struct HashOfHash {
     recent_blockhash: [u8; 8],
     user: [u8; 32],
     clock: [u8; 1]
}