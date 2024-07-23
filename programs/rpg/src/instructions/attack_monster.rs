use anchor_lang::prelude::*;

use crate::{helpers::spend_action_points, Monster, Player, ATTACK_ACTION_POINTS};

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
        constraint = monster.game == player_account.game
    )]
    pub monster: Box<Account<'info, Monster>>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn run_attack_monster(ctx: Context<AttackMonster>) -> Result<()> {

    let mut did_kill = false;

    let hp_before_attack =  ctx.accounts.monster.hitpoints;
    let hp_after_attack = ctx.accounts.monster.hitpoints.saturating_sub(1);
    let damage_dealt = hp_before_attack - hp_after_attack;
    ctx.accounts.monster.hitpoints = hp_after_attack;

        

    if hp_before_attack > 0 && hp_after_attack == 0 {
        did_kill = true;
    }

    if  damage_dealt > 0 {
        msg!("Damage Dealt: {}", damage_dealt);
    } else {
        msg!("Stop it's already dead!");
    }

    ctx.accounts.player_account.experience = ctx.accounts.player_account.experience.saturating_add(1);
    msg!("+1 EXP");

    if did_kill {
        ctx.accounts.player_account.kills = ctx.accounts.player_account.kills.saturating_add(1);
        msg!("You killed the monster!");
    }

    // Spend 1 lamports to attack monster
    let action_point_to_spend = ATTACK_ACTION_POINTS;

    spend_action_points(
        action_point_to_spend, 
        &mut ctx.accounts.player_account,
        &ctx.accounts.player.to_account_info(), 
        &ctx.accounts.system_program.to_account_info()
    )?;

    Ok(())
}