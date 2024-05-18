use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use anchor_lang::solana_program::log::sol_log_compute_units;

mod state;
mod constants;

use state::*;
use constants::*;

declare_id!("2jqUJr2sQzfNhzpTEnUnMbM2enCs4kQG6YTjmm1SZ9rN");

// ----------- HELPER ----------

pub fn spend_action_points<'info>(
    action_points: u64, 
    player_account: &mut Account<'info, Player>,
    player: &AccountInfo<'info>, 
    system_program: &AccountInfo<'info>, 
) -> Result<()> {

    player_account.action_points_spent = player_account.action_points_spent.checked_add(action_points).unwrap();
    player_account.action_points_to_be_collected = player_account.action_points_to_be_collected.checked_add(action_points).unwrap();

    let cpi_context = CpiContext::new(
        system_program.clone(), 
        Transfer {
            from: player.clone(),
            to: player_account.to_account_info().clone(),
        }
    );
    transfer(cpi_context, action_points)?;

    msg!("Minus {} action points", action_points);

    Ok(())
}

// ----------- CREATE GAME ----------

#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(
        init, 
        seeds=[b"GAME", treasury.key().as_ref()],
        bump,
        payer = game_master, 
        space = std::mem::size_of::<Game>()+ 8
    )]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub game_master: Signer<'info>,
    pub treasury: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn run_create_game(ctx: Context<CreateGame>, max_items_per_player: u8) -> Result<()> {

    ctx.accounts.game.game_master = ctx.accounts.game_master.key().clone();
    ctx.accounts.game.treasury = ctx.accounts.treasury.key().clone();

    ctx.accounts.game.action_points_collected = 0;
    ctx.accounts.game.game_config.max_items_per_player = max_items_per_player;

    msg!("Game created!");

    Ok(())
}

// ----------- CREATE PLAYER ----------
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
        space = std::mem::size_of::<Player>() + std::mem::size_of::<InventoryItem>() * game.game_config.max_items_per_player as usize + 8)
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
    let action_points_to_spend = 100;

    spend_action_points(
        action_points_to_spend, 
        &mut ctx.accounts.player_account,
        &ctx.accounts.player.to_account_info(), 
        &ctx.accounts.system_program.to_account_info()
    )?;

    Ok(())
}

// ----------- SPAWN MONSTER ----------
#[derive(Accounts)]
pub struct SpawnMonster<'info> {
    pub game: Box<Account<'info, Game>>,
    #[account(mut,
        has_one = game,
        has_one = player,
    )]
    pub player_account: Box<Account<'info, Player>>,
    #[account(
        init, 
        seeds=[
            b"MONSTER", 
            game.key().as_ref(), 
            player.key().as_ref(),
            player_account.next_monster_index.to_le_bytes().as_ref()
        ], 
        bump, 
        payer = player, 
        space = std::mem::size_of::<Monster>() + 8)
    ]
    pub monster: Account<'info, Monster>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn run_spawn_monster(ctx: Context<SpawnMonster>) -> Result<()> {
    ctx.accounts.monster.player = ctx.accounts.player.key().clone();
    ctx.accounts.monster.game = ctx.accounts.game.key().clone();
    ctx.accounts.monster.hitpoints = 100;

    ctx.accounts.player_account.next_monster_index = ctx.accounts.player_account.next_monster_index.checked_add(1).unwrap();
    msg!("Monster Spawned!");

    // Spend 5 lamports to spawn monster
    let action_point_to_spend = 5;
    spend_action_points(
        action_point_to_spend, 
        &mut ctx.accounts.player_account,
        &ctx.accounts.player.to_account_info(), 
        &ctx.accounts.system_program.to_account_info()
    )?;

    Ok(())
}

// ----------- ATTACK MONSTER ----------
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
    let action_point_to_spend = 1;

    spend_action_points(
        action_point_to_spend, 
        &mut ctx.accounts.player_account,
        &ctx.accounts.player.to_account_info(), 
        &ctx.accounts.system_program.to_account_info()
    )?;

    Ok(())
}

// ----------- REDEEM TO TREASURY ----------
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

#[program]
pub mod rpg {
    use super::*;

    pub fn create_game(ctx: Context<CreateGame>, max_items_per_player: u8) -> Result<()> {
        run_create_game(ctx, max_items_per_player)?;
        sol_log_compute_units();
        Ok(())
    }

    pub fn create_player(ctx: Context<CreatePlayer>) -> Result<()> {
        run_create_player(ctx)?;
        sol_log_compute_units();
        Ok(())
    }

    pub fn spawn_monster(ctx: Context<SpawnMonster>) -> Result<()> {
        run_spawn_monster(ctx)?;
        sol_log_compute_units();
        Ok(())
    }

    pub fn attack_monster(ctx: Context<AttackMonster>) -> Result<()> {
        run_attack_monster(ctx)?;
        sol_log_compute_units();
        Ok(())
    }

    pub fn deposit_action_points(ctx: Context<CollectActionPoints>) -> Result<()> {
        run_collect_action_points(ctx)?;
        sol_log_compute_units();
        Ok(())
    }
}