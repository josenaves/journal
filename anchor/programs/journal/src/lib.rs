#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("2P3oXWsmf5JuGBM6FRo6crd7PERYTH1gns7kgUhPf2GH");

#[program]
pub mod journal {
    use super::*;

  pub fn close(_ctx: Context<CloseJournal>) -> Result<()> {
    Ok(())
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.journal.count = ctx.accounts.journal.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.journal.count = ctx.accounts.journal.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeJournal>) -> Result<()> {
    Ok(())
  }

  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.journal.count = value.clone();
    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitializeJournal<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + Journal::INIT_SPACE,
  payer = payer
  )]
  pub journal: Account<'info, Journal>,
  pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseJournal<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub journal: Account<'info, Journal>,
}

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub journal: Account<'info, Journal>,
}

#[account]
#[derive(InitSpace)]
pub struct Journal {
  count: u8,
}
