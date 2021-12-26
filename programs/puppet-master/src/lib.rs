// #region core
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::sysvar;
use arrayref::array_ref;
use puppet::cpi::accounts::SetData;
use puppet::program::Puppet;
use puppet::{self, Data};
declare_id!("7yF2TQsGsGiwZXfMq9EuePbjFP8Zjo7rVDTbHfBFPbgt");

#[program]
mod puppet_master {
    use super::*;
    pub fn pull_strings(ctx: Context<PullStrings>) -> ProgramResult {
        let cpi_program = ctx.accounts.puppet_program.to_account_info();
        let recent_blockhashes = &ctx.accounts.recent_blockhashes;
        let user = &ctx.accounts.user;
        let jare = &ctx.accounts.jare;
        let data = recent_blockhashes.data.borrow();
        let most_recent = array_ref![data, 8, 8];
        invoke(
            &system_instruction::transfer(user.key, jare.key, 5000000),
            &[
                user.to_account_info().clone(),
                jare.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone(),
            ],
        )?;
        let index = u64::from_le_bytes(*most_recent);
        let cpi_accounts = SetData {
            puppet: ctx.accounts.puppet.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        puppet::cpi::set_data(cpi_ctx, index)
    }
}

#[derive(Accounts)]
pub struct PullStrings<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Data>,
    pub puppet_program: Program<'info, Puppet>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub jare: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    #[account(address = sysvar::recent_blockhashes::id())]
    recent_blockhashes: UncheckedAccount<'info>,
}
// #endregion core
