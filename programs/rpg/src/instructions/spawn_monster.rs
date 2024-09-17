use anchor_lang::prelude::*;

use crate::{
    spend_action_points, 
    Game, 
    Monster,
    Player, 
    ANCHOR_DISCRIMINATOR,
    error::RpgError
};

#[derive(Accounts)]
pub struct SpawnMonster<'info> {
    pub game: Box<Account<'info, Game>>,
    #[account(
        mut,
        has_one = game,
        has_one = player,
    )]
    pub player_account: Box<Account<'info, Player>>,
    #[account(
        init, 
        seeds = [
            b"MONSTER", 
            game.key().as_ref(), 
            player.key().as_ref(),
            player_account.next_monster_index.to_le_bytes().as_ref()
        ], 
        bump, 
        payer = player, 
        space = ANCHOR_DISCRIMINATOR + Monster::INIT_SPACE
    )]
    pub monster: Account<'info, Monster>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn run_spawn_monster(ctx: Context<SpawnMonster>) -> Result<()> {
    let game = &ctx.accounts.game;
    let player_account = &mut ctx.accounts.player_account;
    let monster = &mut ctx.accounts.monster;
    let player = &ctx.accounts.player;
    let system_program = &ctx.accounts.system_program;

    monster.player = player.key();
    monster.game = game.key();
    monster.hitpoints = 100;

    msg!("Monster Spawned!");

    player_account.next_monster_index = player_account.next_monster_index
        .checked_add(1)
        .ok_or(error!(RpgError::ArithmeticOverflow))?;

    // SOLUTION EDIT:
    let action_points_to_spend = game.game_config.ap_per_monster_spawn;

    spend_action_points(
        action_points_to_spend, 
        player_account,
        &player.to_account_info(), 
        &system_program.to_account_info()
    )?;

    Ok(())
}