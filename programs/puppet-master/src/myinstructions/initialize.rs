use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::sysvar;
use arrayref::array_ref;

use crate::state::*;

#[repr(C)]
#[derive(Accounts)]
#[instruction(puppet_bump: u8, uuid: String)]
pub struct Initialize<'info> {
    // [Buffer.from("rng_house"), provider.wallet.publicKey.toBuffer(), houseObj.operator.toBuffer()]
    //init, seeds=[b"rng_house".as_ref(), &author.key().to_bytes(), &operator.key.to_bytes()], bump=house_bump, space=HOUSE_SIZE, payer=author)]
    #[account(init, seeds = [b"rng_house".as_ref(), & user.key().to_bytes(), & house.key().to_bytes(), uuid.as_bytes()], bump = puppet_bump, space = HOUSE_SIZE, payer = user)]
    pub puppet: Account<'info, Data>,
    pub user: Signer<'info>,

    #[account(address = sysvar::recent_blockhashes::id())]
    pub recent_blockhashes: AccountInfo<'info>,
    #[account(seeds = [b"rng_house".as_ref(), & house.author.to_bytes(), & house.operator.to_bytes()], bump = house.house_bump)]
    pub house: Account<'info, House>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>, puppet_bump: u8, uuid: String) -> ProgramResult {
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
