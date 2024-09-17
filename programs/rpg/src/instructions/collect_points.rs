use anchor_lang::prelude::*;

use crate::{Game, Player, error::RpgError};

#[derive(Accounts)]
pub struct CollectActionPoints<'info> {
    #[account(
        mut,
        has_one = treasury
    )]
    pub game: Box<Account<'info, Game>>,
    #[account(
        mut,
        has_one = game
    )]
    pub player: Box<Account<'info, Player>>,
    #[account(mut)]
    /// CHECK: It's being checked in the game account
    pub treasury: AccountInfo<'info>,
    #[account(mut)]
    pub player_wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn run_collect_action_points(ctx: Context<CollectActionPoints>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let player = &mut ctx.accounts.player;
    let treasury = &ctx.accounts.treasury;
    let player_wallet = &ctx.accounts.player_wallet;

    let transfer_amount: u64 = player.action_points_spent;  // Change this from action_points_to_be_collected

    // Transfer directly from player's wallet to treasury
    anchor_lang::solana_program::program::invoke(
        &anchor_lang::solana_program::system_instruction::transfer(
            &player_wallet.key(),
            &treasury.key(),
            transfer_amount,
        ),
        &[
            player_wallet.to_account_info().clone(),
            treasury.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],
    )?;

    game.action_points_collected = game.action_points_collected
        .checked_add(transfer_amount)
        .ok_or(error!(RpgError::ArithmeticOverflow))?;

    player.action_points_to_be_collected = 0;
    player.action_points_spent = 0;

    msg!("The treasury collected {} action points", transfer_amount);

    Ok(())
}