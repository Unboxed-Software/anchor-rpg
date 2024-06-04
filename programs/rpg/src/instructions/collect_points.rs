use anchor_lang::prelude::*;

use crate::{Game, Player};

#[derive(Accounts)]
pub struct CollectActionPoints<'info> {
    #[account(
        mut,
        has_one=treasury
    )]
    pub game: Box<Account<'info, Game>>,
    #[account(
        mut,
        has_one=game
    )]
    pub player: Box<Account<'info, Player>>,
    #[account(mut)]
    /// CHECK: It's being checked in the game account
    pub treasury: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

// literally anyone who pays for the TX fee can run this command - give it to a clockwork bot
pub fn run_collect_action_points(ctx: Context<CollectActionPoints>) -> Result<()> {
    let transfer_amount: u64 = ctx.accounts.player.action_points_to_be_collected;

    **ctx.accounts.player.to_account_info().try_borrow_mut_lamports()? -= transfer_amount;
    **ctx.accounts.treasury.to_account_info().try_borrow_mut_lamports()? += transfer_amount;

    ctx.accounts.player.action_points_to_be_collected = 0;

    ctx.accounts.game.action_points_collected = ctx.accounts.game.action_points_collected.checked_add(transfer_amount).unwrap();

    msg!("The treasury collected {} action points to treasury", transfer_amount);

    Ok(())
}