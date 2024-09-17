use anchor_lang::prelude::*;

use crate::{
    spend_action_points, 
    Game, 
    Player, 
    ANCHOR_DISCRIMINATOR, 
    NO_EFFECT_FLAG
};

#[derive(Accounts)]
pub struct CreatePlayer<'info> {
    pub game: Box<Account<'info, Game>>,
    #[account(
        init, 
        seeds = [
            b"PLAYER", 
            game.key().as_ref(), 
            player.key().as_ref()
        ], 
        bump, 
        payer = player, 
        space = ANCHOR_DISCRIMINATOR + Player::INIT_SPACE
    )]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn run_create_player(ctx: Context<CreatePlayer>) -> Result<()> {
    let game = &ctx.accounts.game;
    let player_account = &mut ctx.accounts.player_account;
    let player = &ctx.accounts.player;
    let system_program = &ctx.accounts.system_program;

    player_account.player = player.key();
    player_account.game = game.key();

    player_account.status_flag = NO_EFFECT_FLAG;
    player_account.experience = 0;
    player_account.kills = 0;

    msg!("Hero has entered the game!");

    // SOLUTION EDIT:
    let action_points_to_spend = game.game_config.ap_per_player_creation;

    spend_action_points(
        action_points_to_spend, 
        player_account,
        &player.to_account_info(), 
        &system_program.to_account_info()
    )?;

    Ok(())
}