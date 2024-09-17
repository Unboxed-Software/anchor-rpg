use anchor_lang::prelude::*;
use crate::{helpers::spend_action_points, Monster, Player, ATTACK_ACTION_POINTS, error::RpgError};

#[derive(Accounts)]
pub struct AttackMonster<'info> {
    #[account(
        mut,
        has_one = player,
    )]
    pub player_account: Box<Account<'info, Player>>,
    #[account(
        mut,
        has_one = player,
        constraint = monster.game == player_account.game @ RpgError::GameMismatch
    )]
    pub monster: Box<Account<'info, Monster>>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn run_attack_monster(ctx: Context<AttackMonster>) -> Result<()> {
    let player_account = &mut ctx.accounts.player_account;
    let monster = &mut ctx.accounts.monster;

    let hp_before_attack = monster.hitpoints;
    let hp_after_attack = monster.hitpoints.saturating_sub(1);
    let damage_dealt = hp_before_attack.saturating_sub(hp_after_attack);
    monster.hitpoints = hp_after_attack;

    if damage_dealt > 0 {
        msg!("Damage Dealt: {}", damage_dealt);
        player_account.experience = player_account.experience.saturating_add(1);
        msg!("+1 EXP");

        if hp_after_attack == 0 {
            player_account.kills = player_account.kills.saturating_add(1);
            msg!("You killed the monster!");
        }
    } else {
        msg!("Stop it's already dead!");
    }

    // Spend 1 lamport to attack monster
    let action_point_to_spend = ATTACK_ACTION_POINTS;

    spend_action_points(
        action_point_to_spend, 
        player_account,
        &ctx.accounts.player.to_account_info(), 
        &ctx.accounts.system_program.to_account_info()
    )?;

    Ok(())
}