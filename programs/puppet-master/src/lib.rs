use anchor_lang::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::sysvar;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use arrayref::array_ref;

use myinstructions::*;
pub mod myinstructions;

// use crate::constants::constants::HOUSE_SIZE;
// use crate::myerrors::myerrors::MyErrorCode;
// use crate::utils::utils::{calculate_hash, HashOfHash};

// pub mod myinstructions;
// pub mod state;
// pub mod utils;
// pub mod constants;
// pub mod myerrors;

declare_id!("9pJ55KszBGk1Td3LbRrWLszAaiXg7YLW5oouLABJwsZg");

#[program]
mod puppet_master {
    use crate::create_house::CreateHouse;
    use super::*;

    pub fn create_house<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateHouse<'info>>,
        house_bump: u8,
        author_fee_bump: u8,
        operator_treasury_bump: u8,
        operator_fee_bump: u8,
        fee_basis_points: u16,
    ) -> ProgramResult {
        myinstructions::create_house::handler(ctx,
                                            house_bump,
                                            author_fee_bump,
                                            operator_treasury_bump,
                                            operator_fee_bump,
                                            fee_basis_points)
    }

    /*
    pub fn pull_strings(ctx: Context<PullStrings>, bet: u64) -> ProgramResult {
        myinstructions::pull_strings::handler(ctx, bet)
    }

    pub fn uncover(ctx: Context<Uncover>) -> ProgramResult {
        myinstructions::uncover::handler(ctx)
    }

    pub fn initialize(ctx: Context<Initialize>, puppet_bump: u8, uuid: String) -> ProgramResult {
        myinstructions::initialize::handler(ctx, puppet_bump, uuid)
    }

    pub fn author_fee_withdraw(ctx: Context<AuthorFeeWithdraw>, sol: u64) -> ProgramResult {
        myinstructions::author_fee_withdraw::handler(ctx, sol)
    }

    pub fn operator_fee_withdraw(ctx: Context<OperatorFeeWithdraw>, sol: u64) -> ProgramResult {
        myinstructions::operator_fee_withdraw::handler(ctx, sol)
    }

    pub fn operator_treasury_withdraw(ctx: Context<OperatorTreasuryWithdraw>, sol: u64) -> ProgramResult {
        myinstructions::operator_treasury_withdraw::handler(ctx, sol)
    }
    */
}
