use anchor_lang::prelude::*;
use crate::{error::RpgError, Game, Player};

#[derive(Accounts)]
pub struct CollectActionPoints<'info> {
    #[account(
        mut,
        has_one = treasury @ RpgError::InvalidTreasury
    )]
    pub game: Box<Account<'info, Game>>,
    #[account(
        mut,
        has_one = game @ RpgError::PlayerGameMismatch
    )]
    pub player: Box<Account<'info, Player>>,
    #[account(mut)]
    /// CHECK: It's being checked in the game account
    pub treasury: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

// Literally anyone who pays for the TX fee can run this command - give it to a clockwork bot
pub fn run_collect_action_points(ctx: Context<CollectActionPoints>) -> Result<()> {
    let transfer_amount = ctx.accounts.player.action_points_to_be_collected;

    // Transfer lamports from player to treasury
    let player_info = ctx.accounts.player.to_account_info();
    let treasury_info = ctx.accounts.treasury.to_account_info();

    **player_info.try_borrow_mut_lamports()? = player_info
        .lamports()
        .checked_sub(transfer_amount)
        .ok_or(RpgError::InsufficientFunds)?;

    **treasury_info.try_borrow_mut_lamports()? = treasury_info
        .lamports()
        .checked_add(transfer_amount)
        .ok_or(RpgError::ArithmeticOverflow)?;

    ctx.accounts.player.action_points_to_be_collected = 0;

    ctx.accounts.game.action_points_collected = ctx.accounts.game
        .action_points_collected
        .checked_add(transfer_amount)
        .ok_or(RpgError::ArithmeticOverflow)?;

    msg!("The treasury collected {} action points", transfer_amount);

    Ok(())
}