use anchor_lang::prelude::*;

use crate::{helpers::spend_action_points, Game, Player, ANCHOR_DISCRIMINATOR, CREATE_PLAYER_ACTION_POINTS, NO_EFFECT_FLAG};

#[derive(Accounts)]
pub struct CreatePlayer<'info> {
    pub game: Box<Account<'info, Game>>,
    #[account(
        init, 
        seeds=[
            b"PLAYER", 
            game.key().as_ref(), 
            player.key().as_ref()
        ], 
        bump, 
        payer = player, 
        space = ANCHOR_DISCRIMINATOR + Player::INIT_SPACE)
    ]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn run_create_player(ctx: Context<CreatePlayer>) -> Result<()> {

    ctx.accounts.player_account.player = ctx.accounts.player.key().clone();
    ctx.accounts.player_account.game = ctx.accounts.game.key().clone();

    ctx.accounts.player_account.status_flag = NO_EFFECT_FLAG;
    ctx.accounts.player_account.experience = 0;
    ctx.accounts.player_account.kills = 0;

    msg!("Hero has entered the game!");

    // Spend 100 lamports to create player
    let action_points_to_spend = CREATE_PLAYER_ACTION_POINTS;

    spend_action_points(
        action_points_to_spend, 
        &mut ctx.accounts.player_account,
        &ctx.accounts.player.to_account_info(), 
        &ctx.accounts.system_program.to_account_info()
    )?;

    Ok(())
}