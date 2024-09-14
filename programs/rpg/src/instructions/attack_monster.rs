use anchor_lang::prelude::*;

use crate::{
    spend_action_points, Game, InventoryItem, Monster, Player, BRONZE_SPEAR, IS_CURSED_FLAG,
};

#[derive(Accounts)]
pub struct AttackMonster<'info> {
    // SOLUTION EDIT:
    pub game: Box<Account<'info, Game>>,
    #[account(
        mut,
        has_one = player,
        // SOLUTION EDIT:
        has_one = game,
    )]
    pub player_account: Box<Account<'info, Player>>,
    #[account(
        mut,
        has_one = player,
        constraint = monster.game == player_account.game
    )]
    pub monster: Box<Account<'info, Monster>>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn run_attack_monster(ctx: Context<AttackMonster>) -> Result<()> {
    let game = &ctx.accounts.game;
    let player_account = &mut ctx.accounts.player_account;
    let monster = &mut ctx.accounts.monster;
    let player = &ctx.accounts.player;
    let system_program = &ctx.accounts.system_program;

    let hp_before_attack = monster.hitpoints;
    let hp_after_attack = monster.hitpoints.saturating_sub(1);
    let damage_dealt = hp_before_attack - hp_after_attack;
    monster.hitpoints = hp_after_attack;

    let did_kill = hp_before_attack > 0 && hp_after_attack == 0;

    if damage_dealt > 0 {
        msg!("Damage Dealt: {}", damage_dealt);
    } else {
        msg!("Stop it's already dead!");
        player_account.status_flag |= IS_CURSED_FLAG;
    }

    player_account.experience = player_account.experience.saturating_add(1);
    msg!("+1 EXP");

    // SOLUTION EDIT:
    let action_points_to_spend = game.game_config.ap_per_monster_attack;

    spend_action_points(
        action_points_to_spend,
        player_account,
        &player.to_account_info(),
        &system_program.to_account_info(),
    )?;

    if did_kill {
        player_account.kills = player_account.kills.saturating_add(1);
        msg!("You killed the monster!");
        // SOLUTION EDIT:
        if player_account.inventory.len() < game.game_config.max_items_per_player as usize {
            player_account.inventory.push(InventoryItem {
                name: BRONZE_SPEAR.clone(),
                amount: 1,
                for_future_use: [0; 128],
            });
            msg!("You looted the monster!");
        } else {
            msg!("You can't loot the monster, your inventory is full!");
        }
    }

    Ok(())
}
