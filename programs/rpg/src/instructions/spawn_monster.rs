use anchor_lang::prelude::*;

use crate::{helpers::spend_action_points, Game, Monster, Player, SPAWN_MONSTER_ACTION_POINTS, ANCHOR_DISCRIMINATOR};

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
    let monster = &mut ctx.accounts.monster;
    monster.player = ctx.accounts.player.key();
    monster.game = ctx.accounts.game.key();
    monster.hitpoints = 100;

    let player_account = &mut ctx.accounts.player_account;
    player_account.next_monster_index = player_account.next_monster_index.checked_add(1).unwrap();

    msg!("Monster Spawned!");

    // Spend 5 lamports to spawn monster
    let action_point_to_spend = SPAWN_MONSTER_ACTION_POINTS;
    spend_action_points(
        action_point_to_spend,
        player_account,
        &ctx.accounts.player.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
    )?;

    Ok(())
}
